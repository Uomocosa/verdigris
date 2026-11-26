use std::path::PathBuf;

use lele_lint::Entry;
use lele_lint::EntryKind;
use lele_lint::Project;

use crate::constants;

pub fn keyword_files(project: &Project) -> Vec<Entry> {
    let root = project.root.join(constants::KEYWORDS_ROOT);
    let Ok(dir) = std::fs::read_dir(&root) else {
        return Vec::new();
    };
    let mut files = Vec::new();
    for entry in dir {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name() else {
            continue;
        };
        files.push(Entry {
            relative_path: PathBuf::from(constants::KEYWORDS_ROOT).join(name),
            absolute_path: path,
            kind: EntryKind::File,
        });
    }
    files.sort_by(|one, other| one.relative_path.cmp(&other.relative_path));
    files
}

#[cfg(test)]
mod tests {
    use crate::verdigris_linter::VerdigrisLinter;
    use lele_lint::EntryKind;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("__keywords__")).unwrap();
        std::fs::write(
            root.path().join("__keywords__").join("r702_9b_flying.rs"),
            "pub struct Flying;\n",
        )
        .unwrap();
        let project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        let files = VerdigrisLinter::keyword_files(&project);
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].relative_path,
            std::path::PathBuf::from("__keywords__/r702_9b_flying.rs")
        );
        assert!(matches!(files[0].kind, EntryKind::File));
    }

    #[test]
    fn subdirectories_are_skipped() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("__keywords__").join("nested")).unwrap();
        std::fs::write(
            root.path().join("__keywords__").join("r702_9b_flying.rs"),
            "pub struct Flying;\n",
        )
        .unwrap();
        let project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        let files = VerdigrisLinter::keyword_files(&project);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn missing_keywords_root_is_empty() {
        let root = tempfile::tempdir().unwrap();
        let project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        let files = VerdigrisLinter::keyword_files(&project);
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn files_come_back_sorted() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("__keywords__")).unwrap();
        for name in ["trample.rs", "flying.rs", "mod.rs"] {
            std::fs::write(root.path().join("__keywords__").join(name), "").unwrap();
        }
        let project = Project {
            root: root.path().to_path_buf(),
            ..Project::default()
        };
        let files = VerdigrisLinter::keyword_files(&project);
        let names: Vec<String> = files
            .iter()
            .map(|file| {
                file.relative_path.file_name().unwrap_or_default().to_string_lossy().into_owned()
            })
            .collect();
        assert_eq!(names, ["flying.rs", "mod.rs", "trample.rs"]);
    }
}
