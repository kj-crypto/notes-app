use reqwest::{Client};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Serialize;
use regex::Regex;
use reqwest::header::{
    HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE,
    ACCEPT_ENCODING, CONNECTION, CACHE_CONTROL
};
use std::time::Duration;


#[derive(Serialize)]
pub struct OgMeta {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>, // base64
    url: Option<String>,
}

type Username = String;
#[derive(Debug)]
enum PageType {
    Instagram(Username),
    Tiktok(Username),
    General(String),
}

fn dispatch_page_type(url: &str) -> PageType {
    if url.contains("instagram.com") {
        let username = Regex::new(r"https?://(?:www\.)?instagram\.com/([^/?#]+)").unwrap();
        let username = username.captures(url).unwrap().get(1).unwrap().as_str();
        PageType::Instagram(username.to_string())
    } else if url.contains("tiktok.com") {
        let username = Regex::new(r"https?://(?:www\.)?tiktok\.com/@([^/?#]+)").unwrap();
        let username = username.captures(url).unwrap().get(1).unwrap().as_str();
        PageType::Tiktok(username.to_string())
    } else {
        PageType::General(url.to_string())
    }
}

pub fn create_http_client() -> Result<Client, String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9,pl;q=0.8")
    );
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br")
    );
    headers.insert(
        CONNECTION,
        HeaderValue::from_static("keep-alive")
    );
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=0")
    );

    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static("\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\"")
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
    headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
    reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .gzip(true)
        .brotli(true)
        .timeout(Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::default())
        .build()
        .map_err(|e| format!("Failed to build stealthy HTTP browser client: {}", e))
}

#[tauri::command]
pub async fn fetch_og_meta(url: String) -> Result<OgMeta, String> {
    let page_type = dispatch_page_type(&url);

    if cfg!(debug_assertions) {
        println!("OG_META fetching. URL: {}, is {:?}", url, page_type);
    }

    match page_type{
        PageType::Instagram(username) => fetch_instagram_meta(username).await,
        PageType::Tiktok(username) => fetch_tiktok_meta(username).await,
        PageType::General(gen_url) => fetch_general_meta(gen_url).await,
    }
}

async fn parse_url_for_meta(client: &Client, url: &String) -> Result<(Option<String>, Option<String>, Option<String>), String> {
    let body = client.get(url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to parse response body text: {}", e))?;

    // Move HTML parsing and extraction to a blocking thread
    let parsed_result = tauri::async_runtime::spawn_blocking(move || -> Result<_, String> {
        let document = scraper::Html::parse_document(&body);
        if detect_traffic(&document) {
            return Err("Unusual traffic detected".to_string());
        }
        let meta_selector = scraper::Selector::parse("meta").unwrap();
        let mut title = None;
        let mut description = None;
        let mut image_url = None;

        for element in document.select(&meta_selector) {
            let attrs = element.value();
            let key = attrs.attr("property")
                .or_else(|| attrs.attr("name"))
                .unwrap_or("");

            let content = attrs.attr("content").map(|s| s.to_string());
            if let Some(val) = content {
                match key {
                    "og:title" | "twitter:title" => {
                        if title.is_none() { title = Some(val); }
                    },
                    "og:description" | "twitter:description" | "description" => {
                        if description.is_none() { description = Some(val); }
                    },
                    "og:image" | "twitter:image" | "image" => {
                        if image_url.is_none() { image_url = Some(val); }
                    },
                    _ => {}
                }
            }
        }
        if title.is_none() || title.as_ref().unwrap().is_empty() {
            let t_selector = scraper::Selector::parse("title").unwrap();
            if let Some(t_el) = document.select(&t_selector).next() {
                title = Some(t_el.text().collect::<String>().trim().to_string());
            }
        }
        Ok((title, description, image_url))
    })
    .await
    .map_err(|e| e.to_string())?;

    parsed_result
}

async fn fetch_general_meta(url: String) -> Result<OgMeta, String> {
    let client = create_http_client()?;
    let (title, description, image_url) = parse_url_for_meta(&client, &url).await?;
    let image = if let Some(img_url) = &image_url {
        fetch_image_as_base64(&client, img_url).await
    } else {
        None
    };

    Ok(OgMeta {
        title,
        description,
        image,
        url: Some(url),
    })
}

/// Fetches an image from a URL and returns a data URL (data:<content-type>;base64,<base64>).
async fn fetch_image_as_base64(client: &Client, image_url: &str) -> Option<String> {
    let resp = client.get(image_url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let content_type = {
        resp.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string()
    };
    let bytes = resp.bytes().await.ok()?;
    let base64 = STANDARD.encode(&bytes);
    Some(format!("data:{};base64,{}", content_type, base64))
}

async fn fetch_instagram_meta(username: String) -> Result<OgMeta, String> {
    let url = format!("https://www.instagram.com/{}/", username);
    let client = create_http_client()?;
    let (title, description, image_url) = parse_url_for_meta(&client, &url).await?;
    let re = Regex::new(r"(.*?)•").unwrap();
    let title = title.unwrap();
    let title = re.captures(&title).unwrap().get(1).unwrap().as_str().trim().to_string();

    let re = Regex::new(r#"(?s)"(.*?)"$"#).unwrap();
    let description = description.unwrap();
    let description = re.captures(&description).unwrap().get(1).unwrap().as_str().trim().to_string();

    let image_url = image_url.unwrap();

    #[cfg(debug_assertions)]
    {
        println!("Title: {}", title);
        println!("Description: {}", description);
        println!("Image URL: {}", image_url);
    }
    let image = fetch_image_as_base64(&client, &image_url).await;

    Ok(OgMeta {
        title: Some(title),
        description: Some(description),
        image,
        url: Some(url),
    })

}

async fn fetch_tiktok_meta(username: String) -> Result<OgMeta, String> {
    let url = format!("https://www.tiktok.com/@{}", username);
    let client = create_http_client()?;

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?;

    let data_json: serde_json::Value;
    if response.status().is_success() {
        let body = response.text().await.map_err(|e| format!("Failed to parse response body text: {}", e))?;
        let re = Regex::new(r#""webapp.user-detail":(\{.*?\}),"webapp.a-b""#).unwrap();
        let raw_data = re.captures(&body).unwrap().get(1).unwrap().as_str();
        data_json = serde_json::from_str(&raw_data).unwrap();
    } else {
        return Err(format!("Failed to fetch data: {}", response.status()));
    }
    let user = data_json["userInfo"]["user"].as_object().ok_or("Failed to parse userInfo.user")?;
    let title = user["nickname"].as_str().ok_or("Failed to parse nickname")?.to_string();
    let mut description = user["signature"].as_str().ok_or("Failed to parse signature")?.to_string();
    if description.is_empty() {
        description = data_json["shareMeta"]["desc"].as_str().ok_or("Failed to parse shareMeta.desc")?.to_string();
        if let Some(pos) = description.find('-') {
            description = description[..pos].trim().to_string();
        }
    }

    let img_url = user["avatarThumb"].as_str().ok_or("Failed to parse avatarThumb")?.to_string();
    let image = fetch_image_as_base64(&client, &img_url).await;

    #[cfg(debug_assertions)]
    {
        println!("Title: {}", title);
        println!("Description: {}", description);
        println!("Image URL: {}", img_url);
    }

    Ok(OgMeta {
        title: Some(title),
        description: Some(description),
        image,
        url: Some(url),
    })
}

fn detect_traffic(document: &scraper::Html) -> bool {
    let title_selector = scraper::Selector::parse("title").unwrap();
    if let Some(title_element) = document.select(&title_selector).next() {
        let title_text = title_element.text().collect::<String>().to_lowercase();
        if title_text.contains("just a moment")
            || title_text.contains("cloudflare")
            || title_text.contains("attention required")
            || title_text.contains("captcha verification")
            || title_text.contains("access denied")
        {
            return true;
        }
    }
    let cf_selector = scraper::Selector::parse("#cf-challenge, .cf-error-overview, #captcha-container").unwrap();
    if document.select(&cf_selector).next().is_some() {
        return true;
    }

    false
}