use crate::utils::selector::{
    BROKEN_BODY_PHRASES_REGEX, BROKEN_KEYWORDS_REGEX, CF_SELECTOR, H1_SELECTOR,
    INSTAGRAM_SPLASH_IMG_HASH, META_TITLE_DESC_SELECTOR, ROBOTS_SELECTOR,
    SPLASH_SCREEN_IMG_SELECTOR, TITLE_SELECTOR, TRAFFIC_TEXT_REGEX,
};
use sha2::{Digest, Sha256};

pub fn detect_traffic(document: &scraper::Html) -> bool {
    if let Some(title_element) = document.select(&TITLE_SELECTOR).next() {
        let title_text = title_element.text().collect::<String>();
        if TRAFFIC_TEXT_REGEX.is_match(&title_text) {
            return true;
        }
    }
    if document.select(&CF_SELECTOR).next().is_some() {
        return true;
    }

    false
}

pub fn detect_broken_link(body: &str, document: &scraper::Html) -> bool {
    if BROKEN_BODY_PHRASES_REGEX.is_match(body) {
        return true;
    }

    if let Some(title_element) = document.select(&TITLE_SELECTOR).next() {
        let title_text = title_element.text().collect::<String>();
        if BROKEN_KEYWORDS_REGEX.is_match(&title_text) {
            return true;
        }
    }

    for h1_element in document.select(&H1_SELECTOR) {
        let h1_text = h1_element.text().collect::<String>();
        if h1_text.len() < 60 && BROKEN_KEYWORDS_REGEX.is_match(&h1_text) {
            return true;
        }
    }

    for meta_element in document.select(&META_TITLE_DESC_SELECTOR) {
        if let Some(content) = meta_element.value().attr("content") {
            if BROKEN_KEYWORDS_REGEX.is_match(content) {
                return true;
            }
        }
    }

    false
}

pub fn detect_instagram_broken_link(document: &scraper::Html) -> bool {
    let has_robot_meta = document
        .select(&ROBOTS_SELECTOR)
        .next()
        .and_then(|meta_element| meta_element.value().attr("content"))
        .map(|content| content.contains("noarchive") && content.contains("noimageindex"))
        .unwrap_or(false);

    let splash_img_hash = document
        .select(&SPLASH_SCREEN_IMG_SELECTOR)
        .next()
        .and_then(|img_element| img_element.value().attr("src"))
        .map(|src| format!("{:x}", Sha256::digest(src.as_bytes())))
        .unwrap_or(String::from(""));

    !has_robot_meta && splash_img_hash == INSTAGRAM_SPLASH_IMG_HASH
}
