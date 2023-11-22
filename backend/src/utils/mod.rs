pub fn is_hexadecimal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Strip "0x" prefix if present
    let mut s: &str = s;
    if let Some(end) = s.strip_prefix("0x") {
        s = end;
    }

    // Check if the remaining string is non-empty and all characters are valid hex digits
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hexadecimal() {
        assert!(is_hexadecimal("0x1a3F"));
        assert!(is_hexadecimal("0x0"));
        assert!(is_hexadecimal("1234567890abcdefABCDEF"));
    }

    #[test]
    fn test_invalid_hexadecimal() {
        assert!(!is_hexadecimal("1z3F"));
        assert!(!is_hexadecimal("ghijkl"));
        assert!(!is_hexadecimal("123G"));
        assert!(!is_hexadecimal("0x"));
    }

    #[test]
    fn test_empty_string() {
        assert!(!is_hexadecimal(""));
    }

    #[test]
    fn test_mixed_case_hexadecimal() {
        assert!(is_hexadecimal("0xAaBbCcDdEeFf"));
    }

    #[test]
    fn test_numeric_non_hexadecimal() {
        assert!(is_hexadecimal("1234567890"));
    }

    #[test]
    fn test_hexadecimal_with_special_characters() {
        assert!(!is_hexadecimal("1a3F-"));
        assert!(!is_hexadecimal("1a 3F"));
        assert!(!is_hexadecimal("1a3F#"));
    }
}
