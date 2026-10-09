use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, COOKIE, HeaderMap,
    HeaderValue, USER_AGENT,
};
use reqwest::{Client, Url};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tauri::async_runtime::Mutex;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
pub static HOST_SEMAPHORES: OnceLock<Mutex<HashMap<String, Arc<Semaphore>>>> = OnceLock::new();
const MAX_CONCURRENT_REQUESTS_PER_HOST: usize = 40;

fn init_global_http_client() -> Result<(), String> {
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
        HeaderValue::from_static("en-US,en;q=0.9,pl;q=0.8"),
    );
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));

    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            "\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert(
        "sec-ch-ua-platform",
        HeaderValue::from_static("\"Windows\""),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
    headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));

    if let Ok(token) = std::env::var("YOUTUBE_SOCS")
        && !token.is_empty() {
            headers.insert(COOKIE, format!("SOCS={}", token).parse().unwrap());
        }

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .gzip(true)
        .brotli(true)
        .timeout(Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::default())
        .pool_max_idle_per_host(500)
        .pool_idle_timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build high-performance client: {}", e))?;

    HTTP_CLIENT
        .set(client)
        .map_err(|_| "HTTP Client was already initialized".to_string())?;
    Ok(())
}

pub fn get_http_client() -> Client {
    HTTP_CLIENT
        .get()
        .expect("HTTP_CLIENT must be initialized via init_global_client() before use!")
        .clone()
}

pub fn init_global_network_manager() -> Result<(), String> {
    init_global_http_client()?;
    HOST_SEMAPHORES
        .set(Mutex::new(HashMap::new()))
        .map_err(|_| "HOST_SEMAPHORES already initialized".to_string())?;
    Ok(())
}

pub async fn acquire_permit_for_host(url_str: &str) -> Result<OwnedSemaphorePermit, String> {
    let parsed_url = Url::parse(url_str).map_err(|e| format!("Invalid URL format: {}", e))?;

    let host = parsed_url
        .host_str()
        .unwrap_or("unknown_host")
        .to_lowercase();

    let mut semaphores_map = HOST_SEMAPHORES
        .get()
        .expect("HOST_SEMAPHORES not initialized")
        .lock()
        .await;

    let semaphore = semaphores_map
        .entry(host.clone())
        .or_insert_with(|| Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS_PER_HOST)))
        .clone();

    drop(semaphores_map);

    let permit = semaphore
        .acquire_owned()
        .await
        .map_err(|e| format!("Failed to aquire host permit: {}", e))?;

    Ok(permit)
}
