use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RuleViolation;
use lele_lint::config;

use crate::constants;
use crate::rules;

pub fn rule_check(
    _self: &rules::rule_doc_text::RuleDocText,
    project: &Project,
) -> Vec<RuleViolation> {
    let lele_path = project.config_base().join(config::CONFIG_FILENAME);
    let loaded = match rules::comprehensive_rules::ComprehensiveRules::try_from(lele_path.as_path())
    {
        Ok(loaded) => loaded,
        Err(error) => {
            return Vec::from([RuleViolation::from_file(
                lele_path,
                rules::rule_doc_text::RuleDocText::CODE,
                format!("could not load the Comprehensive Rules: {error}"),
            )]);
        }
    };
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
        let mut doc = String::new();
        let mut found = false;
        for item in &file.items {
            if let syn::Item::Fn(system) = item {
                if system.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
                    continue;
                }
                if matches!(system.vis, syn::Visibility::Inherited) {
                    continue;
                }
                if system.sig.ident != stem {
                    continue;
                }
                found = true;
                for attr in &system.attrs {
                    if !attr.path().is_ident("doc") {
                        continue;
                    }
                    if let syn::Meta::NameValue(value) = &attr.meta
                        && let syn::Expr::Lit(literal) = &value.value
                        && let syn::Lit::Str(text) = &literal.lit
                    {
                        doc.push_str(text.value().as_str());
                        doc.push('\n');
                    }
                }
            }
        }
        if !found {
            continue;
        }
        for problem in rules::rule_doc_text::RuleDocText::doc_problems(
            &loaded.texts,
            &parsed.ids,
            doc.as_str(),
        ) {
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::rule_doc_text::RuleDocText::CODE,
                problem,
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
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[verdigris.lint]\ncomprehensive_rules = \"rules.txt\"\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rules.txt"),
            "117.4. If all players pass.\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
            kind: EntryKind::File,
        });
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("/// 117.4. If all players pass.\npub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_doc_text::RuleDocText, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn mismatched_doc_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[verdigris.lint]\ncomprehensive_rules = \"rules.txt\"\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rules.txt"),
            "117.4. If all players pass.\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
            kind: EntryKind::File,
        });
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("/// 117.4 If all players pass.\npub fn r117_4_priority() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_doc_text::RuleDocText, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "VRDG003");
    }

    #[test]
    fn missing_config_key_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("lele.toml"), "[lele.lint]\n").unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_doc_text::RuleDocText, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comprehensive_rules"));
    }

    #[test]
    fn files_failing_vrdg001_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[verdigris.lint]\ncomprehensive_rules = \"rules.txt\"\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rules.txt"),
            "117.4. If all players pass.\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/helpers.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/helpers.rs"),
            kind: EntryKind::File,
        });
        project.parsed_files.insert(
            PathBuf::from("__rules__/helpers.rs"),
            syn::parse_str("/// 117.4. If all players pass.\npub fn helpers() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_doc_text::RuleDocText, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn files_without_the_system_fn_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[verdigris.lint]\ncomprehensive_rules = \"rules.txt\"\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rules.txt"),
            "117.4. If all players pass.\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
            kind: EntryKind::File,
        });
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str("pub fn wrong_name() {}").unwrap(),
        );
        let violations = rule_check(&rules::rule_doc_text::RuleDocText, &project);
        assert_eq!(violations.len(), 0);
    }
}
