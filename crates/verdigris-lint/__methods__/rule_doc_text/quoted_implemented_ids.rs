use std::collections::BTreeSet;

use crate::constants;
use crate::rules;

pub fn quoted_implemented_ids(doc: &str) -> BTreeSet<String> {
    let mut paragraphs: Vec<String> = Vec::new();
    let mut current = String::new();
    for line in doc.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(current.clone());
                current.clear();
            }
            continue;
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(line.trim());
    }
    if !current.is_empty() {
        paragraphs.push(current);
    }
    let mut cited = BTreeSet::new();
    let mut implemented_open = true;
    let mut references_open = false;
    for paragraph in &paragraphs {
        if let Some(title) = paragraph.strip_prefix('#') {
            implemented_open = false;
            references_open = title.trim_start_matches('#').trim() == constants::RESTATES_HEADING;
            continue;
        }
        if !implemented_open && !references_open {
            continue;
        }
        if let Some(id) = rules::comprehensive_rules::ComprehensiveRules::rule_id_of(paragraph) {
            cited.insert(id);
        }
    }
    cited
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::quoted_implemented_ids;

    #[test]
    fn test_usage() {
        let cited = quoted_implemented_ids(
            "702.9b A creature with flying.\n\n# Restates\n\n702.9a Flying is an evasion ability.\n",
        );
        assert_eq!(
            cited,
            BTreeSet::from(["702_9b".to_string(), "702_9a".to_string()])
        );
    }

    #[test]
    fn see_also_ids_are_not_implemented_here() {
        let cited = quoted_implemented_ids(
            "702.9b A creature with flying.\n\n# See also\n\n509.1 Declare blockers.\n",
        );
        assert_eq!(cited, BTreeSet::from(["702_9b".to_string()]));
    }

    #[test]
    fn other_headings_are_ignored() {
        let cited = quoted_implemented_ids(
            "702.9b A creature with flying.\n\n# Notes\n\n509.1 Declare blockers.\n",
        );
        assert_eq!(cited, BTreeSet::from(["702_9b".to_string()]));
    }

    #[test]
    fn prose_paragraphs_contribute_no_ids() {
        let cited = quoted_implemented_ids("Reminder text: it flies.\n");
        assert_eq!(cited, BTreeSet::new());
    }
}
