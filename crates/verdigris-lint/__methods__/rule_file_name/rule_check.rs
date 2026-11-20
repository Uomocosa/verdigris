use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::constants;
use crate::rules;

pub fn rule_check(
    _self: &rules::rule_file_name::RuleFileName,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for entry in &project.entries {
        if entry.kind != EntryKind::File {
            continue;
        }
        if entry.relative_path.parent() != Some(Path::new(constants::RULES_ROOT)) {
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
        if let Err(error) = rules::rule_file_stem::RuleFileStem::try_from(stem) {
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_file_name::RuleFileName::CODE,
                format!(
                    "{error}; rename the file to match `r<id>(_r<id>)*_<slug>.rs` (`id` is `<digits>_<digits><lowercase letters>`, `slug` is snake_case not starting with `r<digit>`)",
                ),
            ));
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
        let project = Project {
            entries: vec![
                Entry {
                    relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                    absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                    kind: EntryKind::File,
                },
                Entry {
                    relative_path: PathBuf::from("__rules__/helpers.rs"),
                    absolute_path: PathBuf::from("/crate/__rules__/helpers.rs"),
                    kind: EntryKind::File,
                },
            ],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_file_name::RuleFileName, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("helpers"));
        assert!(violations[0].message.contains("r<id>"));
    }

    #[test]
    fn valid_rule_file_names_are_clean() {
        for stem in [
            "r117_4_priority",
            "r304_1_r304_2_name",
            "r305_6a_r305_6b_name",
            "r305_6_r605_3_basic_land_mana_ability",
        ] {
            let project = Project {
                entries: vec![Entry {
                    relative_path: PathBuf::from(format!("__rules__/{stem}.rs")),
                    absolute_path: PathBuf::from(format!("/crate/__rules__/{stem}.rs")),
                    kind: EntryKind::File,
                }],
                ..Project::default()
            };
            assert_eq!(
                rule_check(&rules::rule_file_name::RuleFileName, &project).len(),
                0,
                "{stem}"
            );
        }
    }

    #[test]
    fn invalid_rule_file_names_are_violations() {
        for stem in [
            "helpers",
            "r117_priority",
            "r117_4",
            "r117_4_R2_x",
            "rule_117_4_x",
        ] {
            let project = Project {
                entries: vec![Entry {
                    relative_path: PathBuf::from(format!("__rules__/{stem}.rs")),
                    absolute_path: PathBuf::from(format!("/crate/__rules__/{stem}.rs")),
                    kind: EntryKind::File,
                }],
                ..Project::default()
            };
            let violations = rule_check(&rules::rule_file_name::RuleFileName, &project);
            assert_eq!(violations.len(), 1, "{stem}");
            assert_eq!(
                violations[0].code,
                rules::rule_file_name::RuleFileName::CODE
            );
        }
    }

    #[test]
    fn non_rule_files_are_skipped() {
        let project = Project {
            entries: vec![
                Entry {
                    relative_path: PathBuf::from("__rules__/mod.rs"),
                    absolute_path: PathBuf::from("/crate/__rules__/mod.rs"),
                    kind: EntryKind::File,
                },
                Entry {
                    relative_path: PathBuf::from("__rules__/__basic__/kinds.rs"),
                    absolute_path: PathBuf::from("/crate/__rules__/__basic__/kinds.rs"),
                    kind: EntryKind::File,
                },
                Entry {
                    relative_path: PathBuf::from("__rules__/notes.md"),
                    absolute_path: PathBuf::from("/crate/__rules__/notes.md"),
                    kind: EntryKind::File,
                },
                Entry {
                    relative_path: PathBuf::from("src/lib.rs"),
                    absolute_path: PathBuf::from("/crate/src/lib.rs"),
                    kind: EntryKind::File,
                },
            ],
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_file_name::RuleFileName, &project);
        assert_eq!(violations.len(), 0);
    }
}
