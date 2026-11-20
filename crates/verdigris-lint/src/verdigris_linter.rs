use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use lele_lint::ErrorFormat;
use lele_lint::Linter;
use lele_lint::PathExemption;
use lele_lint::Project;

use crate::constants;

#[derive(Deref, DerefMut)]
pub struct VerdigrisLinter(pub Linter);

impl Default for VerdigrisLinter {
    fn default() -> Self {
        Self::from(ErrorFormat::Clippy)
    }
}

#[atomic_delegates]
impl From<ErrorFormat> for VerdigrisLinter {
    fn from(error_format: ErrorFormat) -> Self {}
}

#[rustfmt::skip]
impl VerdigrisLinter {
    #[must_use]
    pub fn exemptions() -> Vec<PathExemption> { Vec::from(constants::EXEMPTIONS) }
}

#[atomic_delegates]
impl VerdigrisLinter {
    pub fn include_rules_root(project: &mut Project) -> Result<(), lele_lint::Error> {}
}

#[cfg(test)]
mod tests {
    use super::VerdigrisLinter;
    use lele_lint::ErrorFormat;

    #[test]
    fn test_usage() {
        let linter = VerdigrisLinter::from(ErrorFormat::Clippy);
        assert!(linter.rules.iter().any(|rule| rule.code().starts_with("LELE0")));
        assert!(linter.rules.iter().any(|rule| rule.code().starts_with("LELEBEVY")));
        assert_eq!(linter.exemptions.len(), VerdigrisLinter::exemptions().len());
        assert!(!linter.exemptions.is_empty());
        assert!(linter.exemptions.iter().any(|exemption| exemption.code == "LELE034"));
        assert!(linter.rules.iter().any(|rule| rule.code() == "VRDG001"));
    }
}
