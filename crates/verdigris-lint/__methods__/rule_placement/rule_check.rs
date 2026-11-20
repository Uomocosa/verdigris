use std::collections::HashMap;
use std::path::PathBuf;

use lele_lint::EntryKind;
use lele_lint::Project;
use lele_lint::RuleViolation;
use syn::visit::Visit;

use crate::rules;

pub fn rule_check(
    _self: &rules::rule_placement::RulePlacement,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut known: HashMap<&PathBuf, &PathBuf> = HashMap::new();
    for entry in &project.entries {
        if entry.kind == EntryKind::File {
            known.insert(&entry.relative_path, &entry.absolute_path);
        }
    }
    let mut sources: Vec<(PathBuf, PathBuf)> = known
        .iter()
        .map(|(relative, absolute)| ((*relative).clone(), (*absolute).clone()))
        .collect();
    for relative in project.parsed_files.keys().chain(project.methods_parsed_files.keys()) {
        if known.contains_key(relative) {
            continue;
        }
        sources.push((relative.clone(), project.root.join(relative)));
    }
    let mut violations = Vec::new();
    for (relative, absolute) in &sources {
        if relative.components().any(|component| {
            matches!(
                component.as_os_str().to_str(),
                Some("__rules__" | "rule_examples" | "tests")
            )
        }) {
            continue;
        }
        if relative.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let Some(stem) = relative.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if stem == "mod" {
            continue;
        }
        if rules::rule_file_stem::RuleFileStem::try_from(stem).is_ok() {
            violations.push(RuleViolation::from_file(
                absolute.clone(),
                rules::rule_placement::RulePlacement::CODE,
                format!("rule file {stem}.rs must live in __rules__/{stem}.rs"),
            ));
            continue;
        }
        let file = project
            .parsed_files
            .get(relative)
            .or_else(|| project.methods_parsed_files.get(relative));
        let Some(file) = file else {
            continue;
        };
        let mut scan = rules::rule_placement_scan::RulePlacementScan {
            violations: Vec::new(),
            file: absolute.clone(),
        };
        scan.visit_file(file);
        violations.extend(scan.violations);
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
        let mut project = Project {
            entries: vec![
                Entry {
                    relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                    absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                    kind: EntryKind::File,
                },
                Entry {
                    relative_path: PathBuf::from("src/rules_plugin.rs"),
                    absolute_path: PathBuf::from("/crate/src/rules_plugin.rs"),
                    kind: EntryKind::File,
                },
            ],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("pub fn r117_4_priority() {}").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("src/rules_plugin.rs"),
            syn::parse_str(
                "pub fn add_priority_systems() { rules::r117_4_priority::r117_4_priority(); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn rule_named_fn_in_src_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("src/rules_plugin.rs"),
                absolute_path: PathBuf::from("/crate/src/rules_plugin.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("src/rules_plugin.rs"),
            syn::parse_str("pub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("__rules__"));
    }

    #[test]
    fn rule_named_method_in_src_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("src/rules_plugin.rs"),
                absolute_path: PathBuf::from("/crate/src/rules_plugin.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("src/rules_plugin.rs"),
            syn::parse_str("impl RulesPlugin { pub fn r117_4_priority() {} }").unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("__rules__"));
    }

    #[test]
    fn rule_named_file_in_src_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("src/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/src/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("src/r117_4_priority.rs"),
            syn::parse_str("pub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("__rules__"));
    }

    #[test]
    fn rule_named_fn_inside_rules_is_clean() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("pub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_fn_in_tests_module_is_clean() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("src/rules_plugin.rs"),
                absolute_path: PathBuf::from("/crate/src/rules_plugin.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("src/rules_plugin.rs"),
            syn::parse_str(
                "#[cfg(test)] mod tests { #[test] fn r117_4_passing_resolves_the_spell() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_placement::RulePlacement, &project);
        assert_eq!(violations.len(), 0);
    }
}
