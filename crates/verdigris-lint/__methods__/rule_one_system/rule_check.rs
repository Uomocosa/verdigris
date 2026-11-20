use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::constants;
use crate::rules;

pub fn rule_check(
    _self: &rules::rule_one_system::RuleOneSystem,
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
        if rules::rule_file_stem::RuleFileStem::try_from(stem).is_err() {
            continue;
        }
        let Some(file) = project.parsed_files.get(&entry.relative_path) else {
            continue;
        };
        let mut systems = Vec::new();
        let mut exposed = Vec::new();
        for item in &file.items {
            if let syn::Item::Fn(found) = item {
                if found.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
                    continue;
                }
                if !matches!(found.vis, syn::Visibility::Inherited) {
                    systems.push(found.sig.ident.to_string());
                }
                continue;
            }
            if let syn::Item::Mod(found) = item
                && found.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            let (label, vis): (String, &syn::Visibility) = match item {
                syn::Item::Mod(found) => (format!("mod {}", found.ident), &found.vis),
                syn::Item::Struct(found) => (format!("struct {}", found.ident), &found.vis),
                syn::Item::Enum(found) => (format!("enum {}", found.ident), &found.vis),
                syn::Item::Union(found) => (format!("union {}", found.ident), &found.vis),
                syn::Item::Const(found) => (format!("const {}", found.ident), &found.vis),
                syn::Item::Static(found) => (format!("static {}", found.ident), &found.vis),
                syn::Item::Type(found) => (format!("type {}", found.ident), &found.vis),
                syn::Item::Trait(found) => (format!("trait {}", found.ident), &found.vis),
                syn::Item::TraitAlias(found) => (format!("trait {}", found.ident), &found.vis),
                syn::Item::ExternCrate(found) => {
                    (format!("extern crate {}", found.ident), &found.vis)
                }
                syn::Item::Use(found) => (String::from("use"), &found.vis),
                _ => continue,
            };
            if !matches!(vis, syn::Visibility::Inherited) {
                exposed.push(label);
            }
        }
        let mut problems = Vec::new();
        match systems.as_slice() {
            [only] if only == stem => {}
            [only] => problems.push(format!(
                "defines `pub fn {only}` but should define `pub fn {stem}`"
            )),
            [] => problems.push(format!(
                "has no `pub fn` but should define exactly one `pub fn {stem}`"
            )),
            many => problems.push(format!(
                "defines {} `pub fn` items ({}) but should define exactly one `pub fn {stem}`",
                many.len(),
                many.join(", ")
            )),
        }
        for item in &exposed {
            problems.push(format!(
                "exposes `{item}` but should expose only its system `pub fn {stem}`"
            ));
        }
        if !problems.is_empty() {
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_one_system::RuleOneSystem::CODE,
                problems.join("; "),
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
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("pub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn two_pub_fns_are_a_violation() {
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
            syn::parse_str("pub fn r117_4_priority() {} pub fn r117_4_helper() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("r117_4_helper"));
        assert!(violations[0].message.contains("exactly one"));
    }

    #[test]
    fn wrong_name_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r608_2b_illegal_targets.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r608_2b_illegal_targets.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r608_2b_illegal_targets.rs"),
            syn::parse_str("pub fn wrong_name() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("wrong_name"));
        assert!(violations[0].message.contains("r608_2b_illegal_targets"));
    }

    #[test]
    fn extra_pub_struct_is_a_violation() {
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
            syn::parse_str("pub fn r117_4_priority() {} pub struct Extra;").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("Extra"));
        assert!(violations[0].message.contains("r117_4_priority"));
    }

    #[test]
    fn pub_fn_inside_tests_module_is_ignored() {
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
            syn::parse_str(
                "pub fn r117_4_priority() {} #[cfg(test)] mod tests { pub fn helper() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn private_helper_is_allowed() {
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
            syn::parse_str("pub fn r117_4_priority() {} fn helper() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn missing_system_is_a_violation() {
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
            syn::parse_str("fn helper() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("no `pub fn`"));
        assert!(violations[0].message.contains("r117_4_priority"));
    }

    #[test]
    fn files_failing_vrdg001_are_skipped() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/helpers.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/helpers.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/helpers.rs"),
            syn::parse_str("pub fn helpers() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_one_system::RuleOneSystem, &project);
        assert_eq!(violations.len(), 0);
    }
}
