use crate::utils::error_finder::{detect_broken_link, detect_traffic};
use crate::utils::http_client::{acquire_permit_for_host, get_http_client};
use crate::utils::selector::{META_SELECTOR, TITLE_SELECTOR};
use reqwest::StatusCode;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaSourceKey {
    Og,
    Twitter,
    Default,
}

pub type MetaItem = HashMap<MetaSourceKey, String>;

pub trait MetaItemExt {
    fn retrieve(&self, key: MetaSourceKey) -> Option<String>;
    fn resolve(&self) -> Option<String>;
}

impl MetaItemExt for MetaItem {
    fn resolve(&self) -> Option<String> {
        self.get(&MetaSourceKey::Og)
            .or_else(|| self.get(&MetaSourceKey::Twitter))
            .or_else(|| self.get(&MetaSourceKey::Default))
            .cloned()
    }

    fn retrieve(&self, key: MetaSourceKey) -> Option<String> {
        self.get(&key).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct ParsedMeta {
    pub title: MetaItem,
    pub description: MetaItem,
    pub image_url: MetaItem,
}

impl ParsedMeta {
    fn new() -> Self {
        Self {
            title: MetaItem::with_capacity(3),
            description: MetaItem::with_capacity(3),
            image_url: MetaItem::with_capacity(3),
        }
    }
}

/*
* ===========================================================
* Thread Safety html scraper to move between threads
* Safe because this is read-only scraper, no mutation occurs
* -----------------------------------------------------------
*/
pub struct ThreadSafeHtml(pub scraper::Html);

unsafe impl Send for ThreadSafeHtml {}
unsafe impl Sync for ThreadSafeHtml {}

// Deref for original interface compliance
impl Deref for ThreadSafeHtml {
    type Target = scraper::Html;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/*
* ============================================================
*/

pub struct HtmlParser<'a> {
    url: &'a str,
    early_break_detection: bool,
    pub body: String,
    pub document: ThreadSafeHtml,
}

pub trait Extract<'a> {
    fn new(url: &'a str, early_break_detection: bool) -> Self;
    async fn extract_meta(&mut self) -> Result<ParsedMeta, String>;
}

impl<'a> Extract<'a> for HtmlParser<'a> {
    fn new(url: &'a str, early_break_detection: bool) -> Self {
        Self {
            url,
            early_break_detection,
            body: String::new(),
            document: ThreadSafeHtml(scraper::Html::new_document()),
        }
    }

    async fn extract_meta(&mut self) -> Result<ParsedMeta, String> {
        let body;
        {
            let _permit = acquire_permit_for_host(self.url).await?;
            let client = get_http_client();
            let response = client
                .get(self.url)
                .send()
                .await
                .map_err(|e| format!("Broken link. Error: {}", e))?;

            let status = response.status();
            if status == StatusCode::NOT_FOUND || status == StatusCode::GONE {
                return Err(format!(
                    "Broken link detectec. Server returned status {}",
                    status
                ));
            }
            if !status.is_success() {
                return Err(format!("Server returned status {}", status));
            }
            body = response
                .text()
                .await
                .map_err(|e| format!("Failed to parse response body text: {}", e))?;
        }
        self.body = body.clone();
        let early_break = self.early_break_detection;

        // Move HTML parsing and extraction to a blocking thread
        let (meta, parsed_document) =
            tauri::async_runtime::spawn_blocking(move || -> Result<_, String> {
                let document = scraper::Html::parse_document(&body);
                if detect_traffic(&document) {
                    return Err("Unusual traffic detected".to_string());
                }
                if early_break && detect_broken_link(&body, &document) {
                    return Err("Broken link detected".to_string());
                }
                let mut meta = ParsedMeta::new();

                for element in document.select(&META_SELECTOR) {
                    let attrs = element.value();
                    let key = attrs
                        .attr("property")
                        .or_else(|| attrs.attr("name"))
                        .unwrap_or("");

                    let Some(val) = attrs.attr("content").filter(|s| !s.is_empty()) else {
                        continue;
                    };
                    let val = val.to_string();

                    match key {
                        "og:title" => {
                            meta.title.insert(MetaSourceKey::Og, val);
                        }
                        "twitter:title" => {
                            meta.title.insert(MetaSourceKey::Twitter, val);
                        }

                        "description" => {
                            meta.description.insert(MetaSourceKey::Default, val);
                        }
                        "og:description" => {
                            meta.description.insert(MetaSourceKey::Og, val);
                        }
                        "twitter:description" => {
                            meta.description.insert(MetaSourceKey::Twitter, val);
                        }

                        "og:image" => {
                            meta.image_url.insert(MetaSourceKey::Og, val);
                        }
                        "twitter:image" => {
                            meta.image_url.insert(MetaSourceKey::Twitter, val);
                        }
                        "image" => {
                            meta.image_url.insert(MetaSourceKey::Default, val);
                        }

                        _ => {}
                    };
                }
                if meta.title.is_empty() {
                    if let Some(t_el) = document.select(&TITLE_SELECTOR).next() {
                        let text = t_el.text().collect::<String>().trim().to_string();
                        if !text.is_empty() {
                            meta.title.insert(MetaSourceKey::Default, text);
                        }
                    }
                }
                Ok((meta, ThreadSafeHtml(document)))
            })
            .await
            .map_err(|e| e.to_string())??;

        self.document = parsed_document;
        Ok(meta)
    }
}
