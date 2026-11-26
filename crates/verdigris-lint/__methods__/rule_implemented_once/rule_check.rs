use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use lele_lint::EntryKind;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::constants;
use crate::rules;
use crate::verdigris_linter::VerdigrisLinter;

pub fn rule_check(
    _self: &rules::rule_implemented_once::RuleImplementedOnce,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut implemented: BTreeMap<String, PathBuf> = BTreeMap::new();
    for entry in VerdigrisLinter::keyword_files(project) {
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
        let Ok(parsed) = syn::parse_str::<syn::File>(content.as_str()) else {
            continue;
        };
        let Some(doc) = rules::rule_doc_text::RuleDocText::public_item_doc(&parsed) else {
            continue;
        };
        let name = entry
            .relative_path
            .file_name()
            .map_or_else(|| entry.relative_path.clone(), PathBuf::from);
        for id in rules::rule_doc_text::RuleDocText::quoted_implemented_ids(doc.as_str()) {
            implemented.entry(id).or_insert_with(|| name.clone());
        }
    }
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
        for id in &parsed.ids {
            if let Some(keyword_file) = implemented.get(id) {
                violations.push(RuleViolation::from_file(
                    entry.absolute_path.clone(),
                    rules::rule_implemented_once::RuleImplementedOnce::CODE,
                    format!(
                        "rule `{id}` is implemented by `__keywords__/{}`; move or delete this rule",
                        keyword_file.display(),
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
        std::fs::create_dir_all(dir.path().join("__keywords__")).unwrap();
        std::fs::write(
            dir.path().join("__keywords__").join("r702_9b_flying.rs"),
            "/// 702.9b A creature with flying.\n///\n/// # Restates\n///\n/// 702.9a Flying is an evasion ability.\npub struct Flying;\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r702_9b_blockers.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r702_9b_blockers.rs"),
            kind: EntryKind::File,
        });
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("702_9b"));
        assert!(violations[0].message.contains("__keywords__/r702_9b_flying.rs"));
        assert_eq!(
            violations[0].code,
            rules::rule_implemented_once::RuleImplementedOnce::CODE
        );
    }

    #[test]
    fn restated_ids_count_as_implemented() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("__keywords__")).unwrap();
        std::fs::write(
            dir.path().join("__keywords__").join("r702_9b_flying.rs"),
            "/// 702.9b A creature with flying.\n///\n/// # Restates\n///\n/// 702.9a Flying is an evasion ability.\npub struct Flying;\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r702_9a_evasion.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r702_9a_evasion.rs"),
            kind: EntryKind::File,
        });
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("702_9a"));
    }

    #[test]
    fn see_also_ids_are_not_implemented() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("__keywords__")).unwrap();
        std::fs::write(
            dir.path().join("__keywords__").join("r702_9b_flying.rs"),
            "/// 702.9b A creature with flying.\n///\n/// # See also\n///\n/// 509.1 Declare blockers.\npub struct Flying;\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r509_1a_declare_blockers.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r509_1a_declare_blockers.rs"),
            kind: EntryKind::File,
        });
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn unrelated_rule_files_are_clean() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("__keywords__")).unwrap();
        std::fs::write(
            dir.path().join("__keywords__").join("r702_9b_flying.rs"),
            "/// 702.9b A creature with flying.\npub struct Flying;\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/r117_4_priority.rs"),
            kind: EntryKind::File,
        });
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn rules_files_failing_vrdg001_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("__keywords__")).unwrap();
        std::fs::write(
            dir.path().join("__keywords__").join("r702_9b_flying.rs"),
            "/// 702.9b A creature with flying.\npub struct Flying;\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("__rules__/helpers.rs"),
            absolute_path: PathBuf::from("/crate/__rules__/helpers.rs"),
            kind: EntryKind::File,
        });
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn unreadable_keyword_files_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        let violations = rule_check(&rules::rule_implemented_once::RuleImplementedOnce, &project);
        assert_eq!(violations.len(), 0);
    }
}
