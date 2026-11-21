use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleComments;

impl RuleComments {
    pub const NAME: &'static str = "rule_comments";
    pub const CODE: &'static str = "VRDG007";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `__rules__/` file holds only its system fn's doc comment; every other comment is a violation.",
        why: "The doc comment is the rule file's specification next to its system, checked word-for-word by VRDG003; any other comment is a second prose channel that drifts from the code, and a reader can no longer tell which text is normative.",
        fix: "Delete every comment except the `///` doc comment on the `pub fn` system; move its meaning into names, types or the `test_usage` test.",
        bad: rule_examples!("rule_comments/bad"),
        good: rule_examples!("rule_comments/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleComments {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleComments, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleComments {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
