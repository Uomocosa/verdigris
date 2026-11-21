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
        summary: "Every `__rules__/` file keeps a `#[cfg(test)] mod tests` with at least one `#[test]` fn, named for what it tests; `test_usage` is banned.",
        why: "A rule file without a test guards nothing; a test named for what it tests tells the reader which behaviour the rule implements. A rule is not used, it is tested, so `test_usage` says nothing about the behaviour.",
        fix: "Keep a `#[cfg(test)] mod tests` with at least one `#[test]` fn named for what it tests (e.g. `bolt_kills_bears` in `r608_2_r608_2c_resolve.rs`); never name a rule test `test_usage`.",
        bad: rule_examples!("rule_tests/bad"),
        good: rule_examples!("rule_tests/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
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
