use reqwest::{Client};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Serialize;
use regex::Regex;

#[derive(Serialize)]
pub struct OgMeta {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>, // base64
    url: Option<String>,
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";

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

async fn fetch_general_meta(url: String) -> Result<OgMeta, String> {
    let body = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    // Move HTML parsing and extraction to a blocking thread
    let (title, description, image_url) = tauri::async_runtime::spawn_blocking(move || {
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("meta[property^='og:']").unwrap();

        let mut title = None;
        let mut description = None;
        let mut image_url = None;

        for element in document.select(&selector) {
            if let Some(property) = element.value().attr("property") {
                match property {
                    "og:title" => title = element.value().attr("content").map(|s| s.to_string()),
                    "og:description" => description = element.value().attr("content").map(|s| s.to_string()),
                    "og:image" => image_url = element.value().attr("content").map(|s| s.to_string()),
                    _ => {}
                }
            }
        }
        (title, description, image_url)
    })
    .await
    .map_err(|e| e.to_string())?;

    let image = if let Some(img_url) = &image_url {
        fetch_image_as_base64(img_url).await
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
async fn fetch_image_as_base64(image_url: &str) -> Option<String> {
    let resp = reqwest::get(image_url).await.ok()?;
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
    let client = Client::new();

    let body = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let (title, description, image_url) = tauri::async_runtime::spawn_blocking(move || {
        let document = scraper::Html::parse_document(&body);
        let og_selector = scraper::Selector::parse("meta[property^='og:']").unwrap();
        let desc_selector = scraper::Selector::parse("meta[name='description']").unwrap();

        let mut title = None;
        let mut image_url = None;

        for element in document.select(&og_selector) {
            if let Some(property) = element.value().attr("property") {
                match property {
                    "og:title" => title = element.value().attr("content").map(|s| s.to_string()),
                    "og:image" => image_url = element.value().attr("content").map(|s| s.to_string()),
                    _ => {}
                }
            }
        }
        let description = document.select(&desc_selector)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(|s| s.to_string());
        (title, description, image_url)
    })
    .await
    .map_err(|e| e.to_string())?;

    let re = Regex::new(r"(.*?)•").unwrap();
    let title = title.unwrap();
    let title = re.captures(&title).unwrap().get(1).unwrap().as_str().trim().to_string();

    let re = Regex::new(r#"(?s)"(.*?)"$"#).unwrap();
    let description = description.unwrap();
    let description = re.captures(&description).unwrap().get(1).unwrap().as_str().trim().to_string();

    let image_url = image_url.unwrap();

    if cfg!(debug_assertions) {
        println!("Title: {}", title);
        println!("Description: {}", description);
        println!("Image URL: {}", image_url);
    }
    let image = fetch_image_as_base64(&image_url).await;

    Ok(OgMeta {
        title: Some(title),
        description: Some(description),
        image,
        url: Some(url),
    })

}

async fn fetch_tiktok_meta(username: String) -> Result<OgMeta, String> {
    let url = format!("https://www.tiktok.com/@{}", username);
    let client = Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data_json: serde_json::Value;
    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        let re = Regex::new(r#""webapp.user-detail":\{"userInfo":\{"user":\{(.*?)\}{2}"#).unwrap();
        let raw_data = re.captures(&body).unwrap().get(0).unwrap().as_str();
        let raw_data = format!("{{{}", raw_data);
        data_json = serde_json::from_str(&raw_data).unwrap();
    } else {
        return Err(format!("Failed to fetch data: {}", response.status()));
    }

    let user = &data_json["webapp.user-detail"]["userInfo"]["user"];
    let title = user["nickname"].as_str().unwrap().to_string();
    let mut description = user["signature"].as_str().unwrap().to_string();
    if description.is_empty() {
        description = data_json["webapp.user-detail"]["shareMeta"]["desc"].as_str().unwrap().to_string();
        if let Some(pos) = description.find('-') {
            description = description[..pos].trim().to_string();
        }
    }

    let img_url = user["avatarThumb"].as_str().unwrap().to_string();
    let image = fetch_image_as_base64(&img_url).await;

    if cfg!(debug_assertions) {
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
