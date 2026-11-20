use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleTests;

impl RuleTests {
    pub const NAME: &'static str = "rule_tests";
    pub const CODE: &'static str = "VRDG005";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `__rules__/` file keeps a `#[cfg(test)] mod tests` with a `test_usage` test, and every other `#[test]` fn is named `r<id>_<description>` for one of the file name's rule ids.",
        why: "The test name says which Comprehensive Rules section the scenario checks, so a failing test points at its rule and the `__rules__/` listing stays scannable; an unprefixed or foreign-prefixed test name hides which rule it guards.",
        fix: "Keep `#[test] fn test_usage` and name every other test `r<id>_<snake_case description>` where `<id>` is one of the file name's ids (e.g. `r608_2_bolt_kills_bears` in `r608_2_r608_2c_resolve.rs`).",
        bad: rule_examples!("rule_tests/bad"),
        good: rule_examples!("rule_tests/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(RuleTests, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleTests {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
