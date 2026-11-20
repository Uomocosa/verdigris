use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug)]
pub struct ComprehensiveRules {
    pub source: PathBuf,
    pub texts: BTreeMap<String, String>,
}

#[atomic_delegates]
impl TryFrom<&Path> for ComprehensiveRules {
    type Error = rules::comprehensive_rules_error::ComprehensiveRulesError;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {}
}

#[atomic_delegates]
impl ComprehensiveRules {
    pub fn parse_texts(content: &str) -> BTreeMap<String, String> {}
    pub fn rule_id_of(line: &str) -> Option<String> {}
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::ComprehensiveRules;

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
        let rules = ComprehensiveRules::try_from(dir.path().join("lele.toml").as_path()).unwrap();
        assert_eq!(
            rules.texts,
            BTreeMap::from([(
                String::from("117_4"),
                String::from("117.4. If all players pass.")
            )])
        );
        assert_eq!(rules.source, dir.path().join("rules.txt"));
    }
}
