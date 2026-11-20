use std::collections::BTreeMap;

use crate::rules;

pub fn parse_texts(content: &str) -> BTreeMap<String, String> {
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    let mut texts = BTreeMap::new();
    for line in content.lines() {
        let Some(id) = rules::comprehensive_rules::ComprehensiveRules::rule_id_of(line) else {
            continue;
        };
        texts.entry(id).or_insert_with(|| line.to_string());
    }
    texts
}

#[cfg(test)]
mod tests {
    use super::parse_texts;

    #[test]
    fn test_usage() {
        let texts = parse_texts(
            "117.3a The active player receives priority.\n117.4. If all players pass.\n",
        );
        assert_eq!(
            texts.get("117_3a"),
            Some(&String::from("117.3a The active player receives priority."))
        );
        assert_eq!(
            texts.get("117_4"),
            Some(&String::from("117.4. If all players pass."))
        );
    }

    #[test]
    fn leading_bom_is_stripped() {
        let texts = parse_texts(
            "\u{FEFF}Magic: The Gathering Comprehensive Rules\n117.4. If all players pass.\n",
        );
        assert_eq!(texts.len(), 1);
        assert!(texts.contains_key("117_4"));
    }

    #[test]
    fn example_lines_are_not_rules() {
        let texts = parse_texts(
            "Example: If one effect reads \"X,\" the other wins.\n117.4. If all players pass.\n",
        );
        assert_eq!(texts.len(), 1);
        assert!(texts.contains_key("117_4"));
    }

    #[test]
    fn first_line_wins_on_duplicate_ids() {
        let texts = parse_texts("117.4. First.\n117.4. Second.\n");
        assert_eq!(texts.get("117_4"), Some(&String::from("117.4. First.")));
    }
}
