use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleImplementedOnce;

impl RuleImplementedOnce {
    pub const NAME: &'static str = "rule_implemented_once";
    pub const CODE: &'static str = "VRDG014";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A Comprehensive Rules paragraph quoted by a `__keywords__/` file is implemented nowhere else: no `__rules__/` file name lists the same rule.",
        why: "A CR rule is implemented in exactly one place: a keyword file holds every rule that defines or restates its keyword, and `__rules__/` holds only game rules and interactions. A rule quoted by a keyword file and listed by a `__rules__/` file is enforced twice, and the two copies drift.",
        fix: "Move the behaviour into the keyword file's abilities or delete the `__rules__/` file; drop the rule id from the `__rules__/` file name when the keyword file implements it.",
        bad: rule_examples!("rule_implemented_once/bad"),
        good: rule_examples!("rule_implemented_once/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleImplementedOnce {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleImplementedOnce, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleImplementedOnce {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
