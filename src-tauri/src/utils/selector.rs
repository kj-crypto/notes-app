use once_cell::sync::Lazy;
use regex::Regex;
use scraper::Selector;

pub static META_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("meta").unwrap());
pub static TITLE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("title").unwrap());

pub static SPLASH_SCREEN_IMG_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div#splash-screen img").unwrap());

pub const INSTAGRAM_SPLASH_IMG_HASH: &str =
    "dd254b106bcfa7dfe845ae67b6bcce46283829b30c54141ef3a48d799adf5e17";

pub static ROBOTS_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("meta[name='robots']").unwrap());

pub static CF_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("#cf-challenge, .cf-error-overview, #captcha-container").unwrap());

pub static H1_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("h1").unwrap());

pub static META_TITLE_DESC_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("meta[property='og:title'], meta[name='description']").unwrap());

macro_rules! make_regex_from_list {
    ($first:literal $(, $rest:literal)* $(,)?) => {
        Lazy::new(|| {
            Regex::new(concat!(
                "(?i)(",
                $first,
                $( "|", $rest, )*
                ")"
            )).unwrap()
        })
    };
}

pub static TRAFFIC_TEXT_REGEX: Lazy<Regex> = make_regex_from_list![
    "just a moment",
    "cloudflare",
    "attention required",
    "captcha verification",
    "access denied",
];

pub static BROKEN_KEYWORDS_REGEX: Lazy<Regex> = make_regex_from_list![
    "404",
    "not found",
    "page missing",
    "error 404",
    "page deleted",
    "this video isn't available anymore",
];

pub static BROKEN_BODY_PHRASES_REGEX: Lazy<Regex> =
    make_regex_from_list!["this video isn't available anymore",];

pub static INSTAGRAM_USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://(?:www\.)?instagram\.com/([^/?#]+)").unwrap());

pub static TIKTOK_USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://(?:www\.)?tiktok\.com/@([^/?#]+)").unwrap());

pub static INSAGRAM_USER_TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.+?)•").unwrap());

pub static INSAGRAM_USER_DESC_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?s)"(.+?)"$"#).unwrap());

pub static TIKTOK_REHYDRATION_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("script#__UNIVERSAL_DATA_FOR_REHYDRATION__").unwrap());
