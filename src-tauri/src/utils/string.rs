use std::cmp::{max, min};

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1 = s1.chars().collect::<Vec<char>>();
    let v2 = s2.chars().collect::<Vec<char>>();

    let len2 = v2.len();

    let mut prev = (0..=len2).collect::<Vec<usize>>();
    let mut curr = vec![0; len2 + 1];

    for (i, ch1) in v1.iter().enumerate() {
        curr[0] = i + 1;
        for (j, ch2) in v2.iter().enumerate() {
            let cost = if *ch1 == *ch2 { 0 } else { 1 };
            curr[j + 1] = min(min(curr[j] + 1, prev[j + 1] + 1), prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[len2]
}

pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    let n = s1.to_lowercase();
    let u = s2.to_lowercase();

    let distance = levenshtein_distance(&n, &u);
    let max_len = max(n.chars().count(), u.chars().count());

    if max_len == 0 {
        return 1.0;
    }
    (max_len - distance) as f64 / max_len as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_and_empty() {
        assert_eq!(levenshtein_distance("john_doe", "john_doe"), 0);
        assert_eq!(levenshtein_distance("", "tiktok"), 6);
        assert_eq!(levenshtein_distance("user", ""), 4);
        assert_eq!(levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_case_sensitivity_and_spaces() {
        assert_eq!(levenshtein_distance("Alex Smith", "alexsmith"), 3);
        let name1 = "Alex Smith".to_lowercase();
        let name2 = "alexsmith".to_lowercase();
        assert_eq!(levenshtein_distance(&name1, &name2), 1);

        assert_eq!(levenshtein_distance("user_123", "user123"), 1);
    }

    #[test]
    fn test_utf8_and_emoji() {
        assert_eq!(levenshtein_distance("garçon", "garcon"), 1);
        assert_eq!(levenshtein_distance("déjà vu", "deja vu"), 2);
        assert_eq!(levenshtein_distance("user🚀", "user⭐"), 1);
        assert_eq!(levenshtein_distance("🔥", ""), 1);
    }

    #[test]
    fn test_mandarin_unicode() {
        // "你好" (Nǐ hǎo - Hello) -> 2 chars
        // "你好吗" (Nǐ hǎo ma - How are you?) -> 3 chars
        assert_eq!(levenshtein_distance("你好", "你好吗"), 1);
        assert_eq!(levenshtein_distance("TikTok中国", "TikTok台湾"), 2);
    }

    #[test]
    fn test_completely_different() {
        assert_eq!(levenshtein_distance("abc", "xyz"), 3);
        assert_eq!(levenshtein_distance("short", "verylongstring"), 12);
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("john_doe", "john_doe"), 1.0);
        assert_eq!(string_similarity("", "tiktok"), 0.0);
        assert_eq!(string_similarity("user", ""), 0.0);
        assert_eq!(string_similarity("", ""), 1.0);

        let computed = string_similarity("Alex Smith", "alexsmith");
        assert!(computed >= 0.75);
        assert!(computed >= 0.55);

        let computed = string_similarity("user", "user1");
        assert!(computed >= 0.80);
        assert!(computed >= 0.55);

        let computed = string_similarity("john", "user_9801");
        assert_eq!(computed, 0.0);
        assert!(computed < 0.5);
    }
}
