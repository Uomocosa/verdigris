pub fn rule_id_of(line: &str) -> Option<String> {
    let (major, after_major) = line.split_once('.')?;
    if major.is_empty() || !major.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let minor_len = after_major.bytes().take_while(u8::is_ascii_digit).count();
    let minor = after_major.get(..minor_len)?;
    if minor.is_empty() {
        return None;
    }
    let after_minor = after_major.get(minor_len..)?;
    let letters_len = after_minor.bytes().take_while(u8::is_ascii_lowercase).count();
    let letters = after_minor.get(..letters_len)?;
    let after_letters = after_minor.get(letters_len..)?;
    let after_dot = after_letters.strip_prefix('.').unwrap_or(after_letters);
    if !after_dot.starts_with(' ') {
        return None;
    }
    Some(format!("{major}_{minor}{letters}"))
}

#[cfg(test)]
mod tests {
    use super::rule_id_of;

    #[test]
    fn test_usage() {
        assert_eq!(
            rule_id_of("117.3a The active player"),
            Some(String::from("117_3a"))
        );
        assert_eq!(
            rule_id_of("117.4. If all players pass"),
            Some(String::from("117_4"))
        );
        assert_eq!(
            rule_id_of("608.2b If the spell"),
            Some(String::from("608_2b"))
        );
        assert_eq!(
            rule_id_of("510.1. First, the active player"),
            Some(String::from("510_1"))
        );
    }

    #[test]
    fn non_rule_lines_have_no_id() {
        assert_eq!(rule_id_of("Example: If one effect reads"), None);
        assert_eq!(rule_id_of("Introduction"), None);
        assert_eq!(rule_id_of(""), None);
        assert_eq!(rule_id_of("117.4.If all players pass"), None);
        assert_eq!(rule_id_of("r117_4_priority"), None);
    }
}
