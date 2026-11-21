use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RulePlacement;

impl RulePlacement {
    pub const NAME: &'static str = "rule_placement";
    pub const CODE: &'static str = "VRDG006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every rule system lives in `__rules__/`; no fn or file outside `__rules__/` is named like a rule.",
        why: "Rule systems live one-per-file in `__rules__/` so the rule-to-file mapping stays complete; a rule-named fn or file anywhere else splits that mapping, and a reader can no longer find a rule's system by its name.",
        fix: "Move the fn into `__rules__/<name>.rs` as its system, or rename it so it no longer parses as a rule name; move a rule-named file into `__rules__/`.",
        bad: rule_examples!("rule_placement/bad"),
        good: rule_examples!("rule_placement/good"),
    };
}

#[rustfmt::skip]
impl Rule for RulePlacement {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RulePlacement, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RulePlacement {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
