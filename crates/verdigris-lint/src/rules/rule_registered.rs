use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleRegistered;

impl RuleRegistered {
    pub const NAME: &'static str = "rule_registered";
    pub const CODE: &'static str = "VRDG004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `__rules__/` system is declared in `__rules__/mod.rs` and registered on the app outside `__rules__/` as `S::S`.",
        why: "The mod declaration wires the rule file into the crate and the registration wires its system into a RulesPlugin schedule; a rule file missing either never runs, silently dropping its Comprehensive Rules section.",
        fix: "Declare each rule file with `pub mod <stem>;` in `__rules__/mod.rs` and add `<stem>::<stem>` to a RulesPlugin `add_*_systems` fn outside `__rules__/` (a registration inside `#[cfg(test)]` does not count).",
        bad: rule_examples!("rule_registered/bad"),
        good: rule_examples!("rule_registered/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleRegistered {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleRegistered, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleRegistered {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
