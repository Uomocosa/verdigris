use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::constants;
use crate::rules;

const SNIPPET_BEFORE_CHARS: usize = 25;
const SNIPPET_TOTAL_CHARS: usize = 60;

pub fn doc_problems(texts: &BTreeMap<String, String>, ids: &[String], doc: &str) -> Vec<String> {
    let window = |text: &str, position: usize| {
        let start = position.saturating_sub(SNIPPET_BEFORE_CHARS);
        let mut rest = text.chars().skip(start);
        let shown: String = rest.by_ref().take(SNIPPET_TOTAL_CHARS).collect();
        let prefix = if start > 0 { "…" } else { "" };
        let suffix = if rest.next().is_some() { "…" } else { "" };
        format!("{prefix}{shown}{suffix}")
    };
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
    let mut problems = Vec::new();
    let mut documented: BTreeSet<String> = BTreeSet::new();
    let mut implemented_open = true;
    let mut references_open = false;
    for paragraph in &paragraphs {
        if let Some(title) = paragraph.strip_prefix('#') {
            implemented_open = false;
            references_open = [constants::SEE_ALSO_HEADING, constants::RESTATES_HEADING]
                .contains(&title.trim_start_matches('#').trim());
            continue;
        }
        if !implemented_open && !references_open {
            continue;
        }
        let Some(id) = rules::comprehensive_rules::ComprehensiveRules::rule_id_of(paragraph) else {
            if implemented_open {
                problems.push(format!(
                    "doc paragraph does not start with a rule number: {:?}",
                    window(paragraph, 0)
                ));
            }
            continue;
        };
        let Some(expected) = texts.get(&id) else {
            problems.push(format!(
                "doc paragraph starts with unknown rule `{id}`: {:?}",
                window(paragraph, 0)
            ));
            continue;
        };
        if expected != paragraph {
            let position = expected
                .chars()
                .zip(paragraph.chars())
                .position(|(left, right)| left != right)
                .unwrap_or_else(|| expected.chars().count().min(paragraph.chars().count()));
            problems.push(format!(
                "doc text for rule `{id}` differs from the Comprehensive Rules at character {position}: expected {:?} but found {:?}",
                window(expected, position),
                window(paragraph, position)
            ));
        }
        if implemented_open {
            documented.insert(id);
        }
    }
    for id in ids {
        if !documented.contains(id) {
            problems.push(format!(
                "file name lists rule `{id}` but no doc paragraph implements it"
            ));
        }
    }
    for id in &documented {
        if !ids.contains(id) {
            problems.push(format!(
                "doc paragraph implements rule `{id}` but the file name does not list it"
            ));
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::doc_problems;

    #[test]
    fn test_usage() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. If all players pass."),
        )]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4. If all players\npass.\n",
        );
        assert_eq!(problems, Vec::<String>::new());
    }

    #[test]
    fn ascii_quote_mismatch_is_a_problem() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. The stack doesn\u{2019}t resolve."),
        )]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4. The stack doesn't resolve.\n",
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("117_4"));
        assert!(problems[0].contains("character"));
    }

    #[test]
    fn dropped_period_mismatch_is_a_problem() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. If all players pass."),
        )]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4 If all players pass.\n",
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("differs"));
    }

    #[test]
    fn missing_id_is_a_problem() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. If all players pass."),
        )]);
        let problems = doc_problems(&texts, &[String::from("117_4")], "Some prose.\n");
        assert!(problems.iter().any(|problem| problem.contains("does not start")));
        assert!(problems.iter().any(|problem| problem.contains("no doc paragraph")));
    }

    #[test]
    fn extra_id_is_a_problem() {
        let texts = BTreeMap::from([
            (
                String::from("117_3a"),
                String::from("117.3a The active player receives priority."),
            ),
            (
                String::from("117_4"),
                String::from("117.4. If all players pass."),
            ),
        ]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.3a The active player receives priority.\n\n117.4. If all players pass.\n",
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("117_3a"));
        assert!(problems[0].contains("does not list it"));
    }

    #[test]
    fn unknown_number_is_a_problem() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. If all players pass."),
        )]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "999.9. Something unknown.\n\n117.4. If all players pass.\n",
        );
        assert!(problems.iter().any(|problem| problem.contains("unknown rule")));
    }

    #[test]
    fn see_also_text_is_checked_but_not_in_the_id_set() {
        let texts = BTreeMap::from([
            (
                String::from("117_4"),
                String::from("117.4. If all players pass."),
            ),
            (
                String::from("605_1a"),
                String::from("605.1a An activated ability is a mana ability."),
            ),
        ]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4. If all players pass.\n\n# See also\n\n605.1a An activated ability is a mana ability.\n",
        );
        assert_eq!(problems, Vec::<String>::new());
    }

    #[test]
    fn see_also_mismatch_is_a_problem() {
        let texts = BTreeMap::from([
            (
                String::from("117_4"),
                String::from("117.4. If all players pass."),
            ),
            (
                String::from("605_1a"),
                String::from("605.1a An activated ability is a mana ability."),
            ),
        ]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4. If all players pass.\n\n# See also\n\n605.1a A totally different sentence.\n",
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("605_1a"));
    }

    #[test]
    fn late_mismatch_shows_a_window_around_the_difference() {
        let expected = format!("117.3a {}doesn\u{2019}t", "word ".repeat(30));
        let actual = format!("117.3a {}doesn't", "word ".repeat(30));
        let texts = BTreeMap::from([(String::from("117_3a"), expected)]);
        let problems = doc_problems(&texts, &[String::from("117_3a")], actual.as_str());
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("character 162"));
        assert!(problems[0].contains("doesn\u{2019}t"));
        assert!(problems[0].contains("doesn't"));
    }

    #[test]
    fn restates_text_is_checked_but_not_in_the_id_set() {
        let texts = BTreeMap::from([
            (
                String::from("702_9b"),
                String::from("702.9b A creature with flying."),
            ),
            (
                String::from("702_9a"),
                String::from("702.9a Flying is an evasion ability."),
            ),
        ]);
        let problems = doc_problems(
            &texts,
            &[String::from("702_9b")],
            "702.9b A creature with flying.\n\n# Restates\n\n702.9a Flying is an evasion ability.\n",
        );
        assert_eq!(problems, Vec::<String>::new());
    }

    #[test]
    fn restates_mismatch_is_a_problem() {
        let texts = BTreeMap::from([
            (
                String::from("702_9b"),
                String::from("702.9b A creature with flying."),
            ),
            (
                String::from("702_9a"),
                String::from("702.9a Flying is an evasion ability."),
            ),
        ]);
        let problems = doc_problems(
            &texts,
            &[String::from("702_9b")],
            "702.9b A creature with flying.\n\n# Restates\n\n702.9a Flying is totally different.\n",
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("702_9a"));
    }

    #[test]
    fn other_headings_start_unchecked_prose() {
        let texts = BTreeMap::from([(
            String::from("117_4"),
            String::from("117.4. If all players pass."),
        )]);
        let problems = doc_problems(
            &texts,
            &[String::from("117_4")],
            "117.4. If all players pass.\n\n# Notes\n\nFree prose without a number.\n",
        );
        assert_eq!(problems, Vec::<String>::new());
    }
}
