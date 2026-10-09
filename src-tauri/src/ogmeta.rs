use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use crate::utils::http_client::{get_http_client, acquire_permit_for_host};
use crate::utils::meta_parser::{MetaSourceKey, MetaItemExt, HtmlParser, Extract};
use crate::utils::selector::{INSTAGRAM_USERNAME_REGEX, INSAGRAM_USER_TITLE_REGEX, INSAGRAM_USER_DESC_REGEX, TIKTOK_REHYDRATION_SELECTOR, TIKTOK_USERNAME_REGEX};
use crate::utils::error_finder::detect_instagram_broken_link;
use crate::utils::string::string_similarity;

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
    if let Some(username) = INSTAGRAM_USERNAME_REGEX.captures(url)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str()) 
    {
        PageType::Instagram(username.to_string())
    }
    else if let Some(username) = TIKTOK_USERNAME_REGEX.captures(url)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str()) 
    {
        PageType::Tiktok(username.to_string())
    }
    else {
        PageType::General(url.to_string())
    }
}

#[tauri::command]
pub async fn fetch_og_meta(url: String) -> Result<OgMeta, String> {
    let page_type = dispatch_page_type(&url);
    #[cfg(debug_assertions)]
    {
        println!("[OgMeta] fetching url: {}, is {:?}", url, page_type);
    }
    match page_type{
        PageType::Instagram(username) => fetch_instagram_meta(username).await,
        PageType::Tiktok(username) => fetch_tiktok_meta(username).await,
        PageType::General(gen_url) => fetch_general_meta(&gen_url).await,
    }
}

async fn fetch_general_meta(url: &str) -> Result<OgMeta, String> {
    let mut parser = HtmlParser::new(url, true);
    let meta = parser.extract_meta().await?;
    let image = if let Some(img_url) = meta.image_url.resolve() {
        fetch_image_as_base64(&img_url).await
    } else {
        None
    };
    Ok(OgMeta {
        title: meta.title.resolve(),
        description: meta.description.resolve(),
        image,
        url: Some(url.to_string()),
    })
}

/// Fetches an image from a URL and returns a data URL (data:<content-type>;base64,<base64>).
async fn fetch_image_as_base64(image_url: &str) -> Option<String> {
    let client = get_http_client();
    let _permit = acquire_permit_for_host(image_url).await.ok()?;
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
    let mut parser = HtmlParser::new(&url, false);
    let meta = parser.extract_meta().await?;
    let title: Option<String> = meta.title.resolve().map(|raw| {
        INSAGRAM_USER_TITLE_REGEX.captures(&raw)
            .and_then(|cap| cap.get(1))
            .map(|cap| cap.as_str().trim().to_string())
            .unwrap_or(raw)
    });
    let description: Option<String> = meta.description.retrieve(MetaSourceKey::Default).and_then(|raw| {
        INSAGRAM_USER_DESC_REGEX.captures(&raw)
            .and_then(|cap| cap.get(1))
            .map(|cap| cap.as_str().trim().to_string())
    });
    let image = if let Some(img_url) = meta.image_url.resolve() {
        fetch_image_as_base64(&img_url).await
    } else {
        None
    };
    #[cfg(debug_assertions)]
    {
        println!("Title: {:#?}", title);
        println!("Description: {:#?}", description);
        println!("Image URL: {:#?}", meta.image_url.resolve());
    }
    
    if image.is_none() && description.is_none() && title.as_ref().is_some_and(|txt| txt.to_lowercase() == "instagram") && detect_instagram_broken_link(&parser.document){
        return Err("Instagram broken link detected".to_string());
    }

    Ok(OgMeta {
        title,
        description,
        image,
        url: Some(url),
    })
}

/*
* =====================================
* TikTok deserialization structures
* -------------------------------------
*/
#[derive(Deserialize, Debug)]
struct TikTokRehydrationContainer {
    #[serde(rename = "__DEFAULT_SCOPE__")]
    default_scope: Option<TikTokDefaultScope>,
}

#[derive(Deserialize, Debug)]
struct TikTokDefaultScope {
    #[serde(rename = "webapp.user-detail")]
    user_detail: Option<TikTokUserDetail>,
}

#[derive(Deserialize, Debug, Clone)]
struct TikTokUserDetail {
    #[serde(rename = "statusCode")]
    status_code: i32,

    #[serde(rename = "statusMsg")]
    status_msg: Option<String>,
    
    #[serde(rename = "userInfo")]
    user_info: Option<TikTokUserInfo>,
}

#[derive(Deserialize, Debug, Clone)]
struct TikTokUserInfo {
    #[serde(rename = "user")]
    user: TikTokUser,
}

#[derive(Deserialize, Debug, Clone)]
struct TikTokUser {
    #[serde(rename = "nickname")]
    nickname: String,

    #[serde(rename = "uniqueId")]
    unique_id: String,

    #[serde(rename = "signature")]
    description: String,

    #[serde(rename = "avatarThumb")]
    img_url: String,
}
/*
* =================================
*/

impl TikTokUser {
    fn format_title(&self) -> String {
        if string_similarity(&self.nickname.to_lowercase(), &self.unique_id.to_lowercase()) > 0.6 {
            self.nickname.clone()
        } else {
            format!("{} | {}", self.nickname, self.unique_id)
        }
    }

    async fn format_og_meta(&self, url: &str) -> OgMeta {
        let image = fetch_image_as_base64(&self.img_url).await;
        OgMeta {
            title: Some(self.format_title()),
            description: Some(self.description.clone()),
            image,
            url: Some(url.to_string()),
        }
    }
}

async fn fetch_tiktok_meta(username: String) -> Result<OgMeta, String> {
    let url = format!("https://www.tiktok.com/@{}", username);
    let client = get_http_client();
    let body;
    {
        let _permit = acquire_permit_for_host(&url).await?;
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Broken link. Error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch data: {}", response.status()));
        }
        body = response.text().await.map_err(|e| format!("Failed to parse response body text: {}", e))?;
    }

    let user_detail = tauri::async_runtime::spawn_blocking(move || -> Result<TikTokUserDetail, String> {
        let document = scraper::Html::parse_document(&body);

        let json_text = document.select(&TIKTOK_REHYDRATION_SELECTOR)
            .next()
            .map(|element| element.text().collect::<String>())
            .ok_or_else(|| "TikTok rehydration data not founf".to_string())?;
        
            let container: TikTokRehydrationContainer = serde_json::from_str(&json_text)
                .map_err(|e| format!("TikTok deserialize data error: {}", e))?;
            
            let detail = container
                .default_scope
                .ok_or_else(|| "TikTok missing __DEFAULT_SCOPE__".to_string())?
                .user_detail
                .ok_or_else(|| "TikTok missing webapp.user-detail".to_string())?;
            
            Ok(detail)
    })
    .await
    .map_err(|e| e.to_string())??;

    let user_info = user_detail.user_info.ok_or_else(|| {
        let msg = user_detail.status_msg.unwrap_or_default();
        if user_detail.status_code == 10221 {
            return format!("TikTok broken link detected: {}", msg);
        }
        format!("TikTok missing userInfo. statusCode: {} statusMsg: '{}'", user_detail.status_code, msg)
    })?;
    #[cfg(debug_assertions)]
    {
        println!("Title: {} | {}", user_info.user.nickname, user_info.user.unique_id);
        println!("Description: {}", user_info.user.description);
        println!("Image URL: {}", user_info.user.img_url);
    }
    Ok(user_info.user.format_og_meta(&url).await)
}
