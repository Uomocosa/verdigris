use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleOneSystem;

impl RuleOneSystem {
    pub const NAME: &'static str = "rule_one_system";
    pub const CODE: &'static str = "VRDG002";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `__rules__/` file holds exactly one `pub fn` Bevy system named after the file stem.",
        why: "One file maps to one Comprehensive Rules section, and the file name already says which one; a second system or a differently named fn breaks that mapping, and any other public item leaks a second entry point next to the system.",
        fix: "Keep exactly one `pub` fn and name it after the file (`r608_2b_illegal_targets.rs` holds `pub fn r608_2b_illegal_targets`); keep helpers private and move any other public item out of the rule file.",
        bad: rule_examples!("rule_one_system/bad"),
        good: rule_examples!("rule_one_system/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleOneSystem {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleOneSystem, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleOneSystem {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
