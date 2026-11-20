use lele_bevy_lint::BevyLinter;
use lele_lint::ErrorFormat;
use lele_lint::Linter;
use lele_lint::traits::Rule;

use crate::rules;
use crate::verdigris_linter::VerdigrisLinter;

pub fn from_error_format(error_format: ErrorFormat) -> VerdigrisLinter {
    let mut linter = Linter {
        error_format,
        ..Linter::default()
    };
    linter.extend_rules(BevyLinter::bevy_rules());
    let mut verdigris_rules: Vec<Box<dyn Rule>> = Vec::new();
    rules::rule_file_name::RuleFileName::register(&mut verdigris_rules);
    rules::rule_one_system::RuleOneSystem::register(&mut verdigris_rules);
    rules::rule_doc_text::RuleDocText::register(&mut verdigris_rules);
    rules::rule_registered::RuleRegistered::register(&mut verdigris_rules);
    linter.extend_rules(verdigris_rules);
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
        assert!(linter.rules.iter().any(|rule| rule.code() == "VRDG001"));
        assert!(linter.rules.iter().any(|rule| rule.code() == "VRDG002"));
        assert!(linter.rules.iter().any(|rule| rule.code() == "VRDG003"));
        assert!(linter.rules.iter().any(|rule| rule.code() == "VRDG004"));
    }
}
