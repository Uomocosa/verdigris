use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RuleViolation;
use syn::visit::Visit;

use crate::constants;
use crate::rules;

pub fn rule_check(
    _self: &rules::rule_registered::RuleRegistered,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut stems = BTreeMap::new();
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
        if rules::rule_file_stem::RuleFileStem::try_from(stem).is_err() {
            continue;
        }
        stems.insert(stem.to_string(), entry.absolute_path.clone());
    }
    let mut scan = rules::rule_registered_scan::RuleRegisteredScan {
        declared: BTreeSet::new(),
        registered: BTreeSet::new(),
    };
    let mod_path = Path::new(constants::RULES_ROOT).join("mod.rs");
    if let Some(mod_file) = project.parsed_files.get(&mod_path) {
        scan.collect_declared(mod_file);
    }
    for (relative, file) in project.parsed_files.iter().chain(project.methods_parsed_files.iter()) {
        if relative.starts_with(constants::RULES_ROOT) {
            continue;
        }
        for item in &file.items {
            if let syn::Item::Fn(found) = item
                && found.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            if let syn::Item::Mod(found) = item
                && found.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            scan.visit_item(item);
        }
    }
    let mut violations = Vec::new();
    for (stem, absolute) in &stems {
        if !scan.declared.is_empty() && !scan.declared.contains(stem) {
            violations.push(RuleViolation::from_file(
                absolute.clone(),
                rules::rule_registered::RuleRegistered::CODE,
                format!("rule file {stem} is not declared in __rules__/mod.rs"),
            ));
        }
        if !scan.registered.contains(stem) {
            violations.push(RuleViolation::from_file(
                absolute.clone(),
                rules::rule_registered::RuleRegistered::CODE,
                format!(
                    "system {stem}::{stem} is never registered outside __rules__ (add it to a RulesPlugin add_*_systems)"
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
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r117_4_priority;").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str(
                "pub fn add_priority_systems(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn registration_in_src_parsed_files_is_ok() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r117_4_priority;").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("src/rules_plugin.rs"),
            syn::parse_str(
                "pub fn add(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn missing_mod_declaration_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r608_2b_illegal_targets;").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str(
                "pub fn add(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("not declared in __rules__/mod.rs"));
    }

    #[test]
    fn never_registered_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r117_4_priority;").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str("pub fn add(app: &mut App) {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority::r117_4_priority"));
        assert!(violations[0].message.contains("never registered"));
    }

    #[test]
    fn registration_inside_cfg_test_module_does_not_count() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r117_4_priority;").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str(
                "#[cfg(test)] mod tests { pub fn probe() { let _ = rules::r117_4_priority::r117_4_priority; } }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("never registered"));
    }

    #[test]
    fn unrelated_mod_declarations_are_ignored() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str(
                "pub mod r117_4_priority;\npub mod helpers;\n#[path = \"__basic__/mod.rs\"] mod basic;",
            )
            .unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str(
                "pub fn add(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn files_failing_vrdg001_are_skipped() {
        let mut project = Project {
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
        project.parsed_files.insert(
            PathBuf::from("__rules__/mod.rs"),
            syn::parse_str("pub mod r117_4_priority;").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("rules_plugin/add_priority_systems.rs"),
            syn::parse_str(
                "pub fn add(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_registered::RuleRegistered, &project);
        assert_eq!(violations.len(), 0);
    }
}
