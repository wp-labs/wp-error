use unicode_segmentation::UnicodeSegmentation;
/// Return the first 30 Unicode grapheme clusters of the given string.
/// This is a safe, user-facing truncation (not a logical 'split').
pub fn split_string(s: &str) -> String {
    s.graphemes(true).take(30).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string_short() {
        let s = "hello";
        assert_eq!(split_string(s), "hello");
    }

    #[test]
    fn test_split_string_exact_30() {
        let s = "123456789012345678901234567890"; // exactly 30 chars
        assert_eq!(split_string(s), s);
    }

    #[test]
    fn test_split_string_longer_than_30() {
        let s = "1234567890123456789012345678901234567890"; // 40 chars
        let result = split_string(s);
        assert_eq!(result.len(), 30);
        assert_eq!(result, "123456789012345678901234567890");
    }

    #[test]
    fn test_split_string_empty() {
        assert_eq!(split_string(""), "");
    }

    #[test]
    fn test_split_string_unicode() {
        // Chinese characters (each is one grapheme cluster)
        let s = "中文测试字符串用于验证功能是否正确工作这里还需要更多字符来测试"; // more than 30 chars
        let result = split_string(s);
        assert_eq!(result.graphemes(true).count(), 30);
    }

    #[test]
    fn test_split_string_emoji() {
        // Each emoji is one grapheme cluster
        let s = "😀😁😂🤣😃😄😅😆😉😊😋😎😍😘🥰😗😙😚☺🙂🤗🤩🤔🤨😐😑😶🙄😏😣😥";
        let result = split_string(s);
        assert_eq!(result.graphemes(true).count(), 30);
    }

    #[test]
    fn test_split_string_mixed() {
        let s = "Hello世界🌍Test";
        let result = split_string(s);
        assert_eq!(result, "Hello世界🌍Test");
    }
}
