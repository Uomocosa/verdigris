use std::path::Path;

use crate::rules;

pub fn try_from_path(
    path: &Path,
) -> Result<
    rules::comprehensive_rules::ComprehensiveRules,
    rules::comprehensive_rules_error::ComprehensiveRulesError,
> {
    let content = std::fs::read_to_string(path).map_err(|source| {
        rules::comprehensive_rules_error::ComprehensiveRulesError::LeleUnreadable {
            path: path.to_path_buf(),
            source,
        }
    })?;
    let table: toml::Value = content.parse().map_err(|source| {
        rules::comprehensive_rules_error::ComprehensiveRulesError::BadTable {
            path: path.to_path_buf(),
            source,
        }
    })?;
    let relative = table
        .get("verdigris")
        .and_then(|section| section.get("lint"))
        .and_then(|section| section.get("comprehensive_rules"))
        .and_then(toml::Value::as_str)
        .ok_or_else(
            || rules::comprehensive_rules_error::ComprehensiveRulesError::MissingKey {
                path: path.to_path_buf(),
            },
        )?;
    let dir = path.parent().unwrap_or_else(|| Path::new(""));
    let source = dir.join(relative);
    let content = std::fs::read_to_string(&source).map_err(|origin| {
        rules::comprehensive_rules_error::ComprehensiveRulesError::RulesUnreadable {
            path: source.clone(),
            source: origin,
        }
    })?;
    Ok(rules::comprehensive_rules::ComprehensiveRules {
        source,
        texts: rules::comprehensive_rules::ComprehensiveRules::parse_texts(content.as_str()),
    })
}

#[cfg(test)]
mod tests {
    use super::try_from_path;

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
        let rules = try_from_path(dir.path().join("lele.toml").as_path()).unwrap();
        assert_eq!(rules.texts.len(), 1);
        assert_eq!(
            rules.texts.get("117_4"),
            Some(&String::from("117.4. If all players pass."))
        );
    }

    #[test]
    fn missing_table_key_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("lele.toml"), "[lele.lint]\n").unwrap();
        let error = try_from_path(dir.path().join("lele.toml").as_path()).unwrap_err();
        assert!(error.to_string().contains("comprehensive_rules"));
    }

    #[test]
    fn unreadable_rules_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[verdigris.lint]\ncomprehensive_rules = \"missing.txt\"\n",
        )
        .unwrap();
        let error = try_from_path(dir.path().join("lele.toml").as_path()).unwrap_err();
        assert!(error.to_string().contains("missing.txt"));
    }

    #[test]
    fn missing_lele_toml_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let error = try_from_path(dir.path().join("lele.toml").as_path()).unwrap_err();
        assert!(error.to_string().contains("lele.toml"));
    }
}
