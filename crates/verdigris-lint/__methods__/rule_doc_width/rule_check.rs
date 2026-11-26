use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::constants;
use crate::rules;
use crate::verdigris_linter::VerdigrisLinter;

pub fn rule_check(
    _self: &rules::rule_doc_width::RuleDocWidth,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let keyword_files = VerdigrisLinter::keyword_files(project);
    for entry in project.entries.iter().chain(keyword_files.iter()) {
        if entry.kind != EntryKind::File {
            continue;
        }
        if entry.relative_path.parent().is_none_or(|parent| {
            parent != Path::new(constants::RULES_ROOT)
                && parent != Path::new(constants::KEYWORDS_ROOT)
        }) {
            continue;
        }
        let is_source = entry.relative_path.extension().is_some_and(|extension| extension == "rs");
        if !is_source {
            continue;
        }
        let Some(stem) = entry.relative_path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if stem == "mod" {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&entry.absolute_path) else {
            continue;
        };
        for (index, line) in content.lines().enumerate() {
            if !line.trim_start().starts_with("///") {
                continue;
            }
            let width = line.chars().count();
            if width > rules::rule_doc_width::RuleDocWidth::MAX_COLUMNS {
                let line_number = index.saturating_add(1);
                violations.push(RuleViolation::from_site(
                    entry.absolute_path.clone(),
                    line_number,
                    0,
                    rules::rule_doc_width::RuleDocWidth::CODE,
                    format!(
                        "doc comment line {line_number} is {width} columns wide; wrap it to {} columns or less",
                        rules::rule_doc_width::RuleDocWidth::MAX_COLUMNS,
                    ),
                ));
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lele_lint::Entry;
    use lele_lint::EntryKind;
    use lele_lint::Project;

    use super::rule_check;
    use crate::rules;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(
            &path,
            "/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n",
        )
        .unwrap();
        let project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn long_doc_line_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, format!("/// {}\n", "word ".repeat(30))).unwrap();
        let project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path.clone(),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].code,
            rules::rule_doc_width::RuleDocWidth::CODE
        );
        assert!(violations[0].message.contains("line 1"));
        assert!(violations[0].message.contains("100 columns"));
        assert_eq!(violations[0].file, path);
    }

    #[test]
    fn keyword_files_are_checked_too() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("__keywords__")).unwrap();
        std::fs::write(
            root.path().join("__keywords__").join("r702_9b_flying.rs"),
            format!("/// {}\n", "word ".repeat(30)),
        )
        .unwrap();
        let project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn long_code_lines_are_clean() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, format!("pub fn {}() {{}}\n", "x".repeat(200))).unwrap();
        let project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn mod_files_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mod.rs");
        std::fs::write(&path, format!("/// {}\n", "word ".repeat(30))).unwrap();
        let project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/mod.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn exactly_one_hundred_columns_is_clean() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("r117_4_priority.rs");
        let line = format!("/// {}\n", "x".repeat(96));
        assert_eq!(line.trim_end().chars().count(), 100);
        std::fs::write(&path, line.as_str()).unwrap();
        let project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_width::RuleDocWidth, &project);
        assert_eq!(violations.len(), 0);
    }
}
