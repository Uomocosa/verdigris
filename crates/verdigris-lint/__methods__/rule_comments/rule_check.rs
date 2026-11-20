use std::collections::HashSet;
use std::path::Path;

use lele_lint::EntryKind;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RawCode;
use lele_lint::RuleViolation;
use syn::spanned::Spanned;

use crate::constants;
use crate::rules;

pub fn rule_check(
    _self: &rules::rule_comments::RuleComments,
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
        let mut allowed: HashSet<usize> = HashSet::new();
        let mut found = false;
        for item in &file.items {
            let syn::Item::Fn(system) = item else {
                continue;
            };
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
                if attr.path().is_ident("doc") {
                    allowed.insert(attr.span().start().line);
                }
            }
        }
        if !found {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&entry.absolute_path) else {
            continue;
        };
        for hit in RawCode(&content).find_comments() {
            if allowed.contains(&hit.line) {
                continue;
            }
            let line = hit.line;
            violations.push(RuleViolation::from_site(
                entry.absolute_path.clone(),
                line,
                0,
                rules::rule_comments::RuleComments::CODE,
                format!(
                    "comment at line {line}: rule files allow only the system's doc comment (rule text, `# See also`, `# Notes`)"
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
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn line_comment_in_body_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content =
            "/// 117.4. If all players pass.\npub fn r117_4_priority() {\n    // resolve\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 3"));
    }

    #[test]
    fn line_comment_in_tests_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n#[cfg(test)]\nmod tests {\n    // setup\n    #[test]\n    fn test_usage() {}\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 5"));
    }

    #[test]
    fn doc_on_test_fn_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n#[cfg(test)]\nmod tests {\n    /// checks priority\n    #[test]\n    fn test_usage() {}\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 5"));
    }

    #[test]
    fn doc_on_helper_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n/// taps mana\nfn taps() {}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 3"));
    }

    #[test]
    fn block_comment_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content =
            "/// 117.4. If all players pass.\npub fn r117_4_priority() {\n    /* resolve */\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 3"));
    }

    #[test]
    fn inner_doc_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let content =
            "//! module notes\n/// 117.4. If all players pass.\npub fn r117_4_priority() {}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("comment at line 1"));
    }

    #[test]
    fn string_with_slashes_is_clean() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {\n    let text = \"// not a comment\";\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn raw_string_with_slashes_is_clean() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {\n    let text = r#\"//\"#;\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn slash_char_is_clean() {
        let dir = tempfile::tempdir().unwrap();
        let content = "/// 117.4. If all players pass.\npub fn r117_4_priority() {\n    let slash = '/';\n}\n";
        let path = dir.path().join("r117_4_priority.rs");
        std::fs::write(&path, content).unwrap();
        let mut project = Project {
            entries: vec![Entry {
                relative_path: PathBuf::from("__rules__/r117_4_priority.rs"),
                absolute_path: path,
                kind: EntryKind::File,
            }],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("__rules__/r117_4_priority.rs"),
            syn::parse_str(content).unwrap(),
        );
        let violations = rule_check(&rules::rule_comments::RuleComments, &project);
        assert_eq!(violations.len(), 0);
    }
}
