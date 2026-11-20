use lele_bevy_lint::BevyLinter;
use lele_lint::ErrorFormat;
use lele_lint::Linter;

use crate::verdigris_linter::VerdigrisLinter;

pub fn from_error_format(error_format: ErrorFormat) -> VerdigrisLinter {
    let mut linter = Linter {
        error_format,
        ..Linter::default()
    };
    linter.extend_rules(BevyLinter::bevy_rules());
    for exemption in VerdigrisLinter::exemptions() {
        linter.exempt(exemption);
    }
    VerdigrisLinter(linter)
}

#[cfg(test)]
mod tests {
    use super::from_error_format;
    use lele_lint::ErrorFormat;

    #[test]
    fn test_usage() {
        let linter = from_error_format(ErrorFormat::Clippy);
        assert!(linter.rules.iter().any(|rule| rule.code().starts_with("LELEBEVY")));
    }
}
