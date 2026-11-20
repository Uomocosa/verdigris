use std::path::PathBuf;

use lele_lint::Entry;
use lele_lint::Error;
use lele_lint::ModuleInfo;
use lele_lint::Project;

use crate::constants;

pub fn include_rules_root(project: &mut Project) -> Result<(), Error> {
    let root = PathBuf::from(constants::RULES_ROOT);
    let absolute = project.root.join(&root);
    if !absolute.is_dir() {
        return Ok(());
    }
    let inner = Entry::walk(&absolute, &absolute)?;
    let prefixed: Vec<Entry> = inner
        .into_iter()
        .map(|entry| Entry {
            relative_path: root.join(&entry.relative_path),
            absolute_path: entry.absolute_path,
            kind: entry.kind,
        })
        .collect();
    let modules = ModuleInfo::build(&prefixed);
    let (parsed, failures) = Entry::parse_all(&prefixed);
    project.extra_src_dirs.push(absolute);
    project.entries.extend(prefixed);
    project.module_info.extend(modules);
    project.parsed_files.extend(parsed);
    project.parse_failures.extend(failures);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lele_lint::Project;

    use super::include_rules_root;

    #[test]
    fn test_usage() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("__rules__")).unwrap();
        std::fs::write(
            root.path().join("__rules__").join("r100_1_probe.rs"),
            "pub fn r100_1_probe() {}\n",
        )
        .unwrap();
        let mut project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        include_rules_root(&mut project).unwrap();
        assert!(project.parsed_files.contains_key(&PathBuf::from("__rules__/r100_1_probe.rs")));
        assert_eq!(project.extra_src_dirs.len(), 1);
    }

    #[test]
    fn test_usage_crate_without_rules_root_is_unchanged() {
        let root = tempfile::tempdir().unwrap();
        let mut project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        include_rules_root(&mut project).unwrap();
        assert_eq!(project.parsed_files.len(), 0);
        assert_eq!(project.extra_src_dirs.len(), 0);
    }
}
