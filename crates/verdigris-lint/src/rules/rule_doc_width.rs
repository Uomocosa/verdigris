use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleDocWidth;

impl RuleDocWidth {
    pub const NAME: &'static str = "rule_doc_width";
    pub const CODE: &'static str = "VRDG015";
    pub const MAX_COLUMNS: usize = 100;
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "In `__rules__/` and `__keywords__/` doc comments, no line is longer than 100 columns.",
        why: "Rule files quote long Comprehensive Rules paragraphs; wrapped at 100 columns the quotation stays readable in a side-by-side diff, and a reader can tell a wrapped line from two rules at a glance.",
        fix: "Wrap the doc comment so every `///` line fits in 100 columns; keep the quoted text verbatim (VRDG003 joins wrapped lines with a single space).",
        bad: rule_examples!("rule_doc_width/bad"),
        good: rule_examples!("rule_doc_width/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleDocWidth {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleDocWidth, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleDocWidth {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
