use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::constants;
use crate::rules;

pub fn rule_check(_self: &rules::rule_tests::RuleTests, project: &Project) -> Vec<RuleViolation> {
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
        let Ok(parsed) = rules::rule_file_stem::RuleFileStem::try_from(stem) else {
            continue;
        };
        let Some(file) = project.parsed_files.get(&entry.relative_path) else {
            continue;
        };
        let tests = file.items.iter().find_map(|item| match item {
            syn::Item::Mod(found)
                if found.ident == "tests"
                    && found.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) =>
            {
                found.content.as_ref().map(|(_, items)| items)
            }
            _ => None,
        });
        let Some(items) = tests else {
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_tests::RuleTests::CODE,
                format!("rule file {stem} has no #[cfg(test)] mod tests"),
            ));
            continue;
        };
        let names: Vec<String> = items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(found)
                    if found.attrs.iter().any(|attr| ItemMarker(attr).is_test()) =>
                {
                    Some(found.sig.ident.to_string())
                }
                _ => None,
            })
            .collect();
        if !names.iter().any(|name| name == "test_usage") {
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_tests::RuleTests::CODE,
                format!("rule file {stem} has no #[test] fn test_usage in its tests module"),
            ));
        }
        let allowed: Vec<String> = parsed.ids.iter().map(|id| format!("r{id}_")).collect();
        for name in &names {
            if name == "test_usage" {
                continue;
            }
            if allowed.iter().any(|prefix| name.starts_with(prefix) && name.len() > prefix.len()) {
                continue;
            }
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_tests::RuleTests::CODE,
                format!(
                    "test {name} in {stem} must be named r<id>_<description> (allowed prefixes: {})",
                    allowed.join(", ")
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
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(
                "pub fn r117_4_priority() {} #[cfg(test)] mod tests { #[test] fn test_usage() {} #[test] fn r117_4_passing_resolves_the_spell() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn missing_tests_module_is_a_violation() {
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
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("no #[cfg(test)] mod tests"));
    }

    #[test]
    fn missing_test_usage_is_a_violation() {
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
                "pub fn r117_4_priority() {} #[cfg(test)] mod tests { #[test] fn r117_4_passing_resolves_the_spell() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r117_4_priority"));
        assert!(violations[0].message.contains("test_usage"));
    }

    #[test]
    fn unprefixed_name_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r608_2_r608_2c_resolve.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r608_2_r608_2c_resolve.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r608_2_r608_2c_resolve.rs"),
            syn::parse_str(
                "pub fn r608_2_r608_2c_resolve() {} #[cfg(test)] mod tests { #[test] fn test_usage() {} #[test] fn test_bolt_kills_bears() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("test_bolt_kills_bears"));
        assert!(violations[0].message.contains("r608_2_"));
        assert!(violations[0].message.contains("r608_2c_"));
    }

    #[test]
    fn foreign_id_prefix_is_a_violation() {
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r305_6_basic_land_mana_ability.rs"),
                absolute_path: PathBuf::from("/crate/__rules__/r305_6_basic_land_mana_ability.rs"),
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r305_6_basic_land_mana_ability.rs"),
            syn::parse_str(
                "pub fn r305_6_basic_land_mana_ability() {} #[cfg(test)] mod tests { #[test] fn test_usage() {} #[test] fn r107_5_tapped_land_cannot_tap_again() {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("r107_5_tapped_land_cannot_tap_again"));
        assert!(violations[0].message.contains("r305_6_"));
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
        let violations = rule_check(&rules::rule_tests::RuleTests, &project);
        assert_eq!(violations.len(), 0);
    }
}
