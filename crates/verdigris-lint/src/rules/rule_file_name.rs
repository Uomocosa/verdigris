use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleFileName;

impl RuleFileName {
    pub const NAME: &'static str = "rule_file_name";
    pub const CODE: &'static str = "VRDG001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `.rs` file directly inside `__rules__/` or `__keywords__/` (except `mod.rs`) is named `r<id>(_r<id>)*_<slug>.rs`.",
        why: "The file name maps each file to the Comprehensive Rules sections it enforces; a shared grammar lets later checks tie the file name to its rule text and keeps the `__rules__/` and `__keywords__/` listings scannable.",
        fix: "Rename the file to `r<id>(_r<id>)*_<slug>.rs`, where each `id` is `<digits>_<digits><lowercase letters>` (e.g. `117_4`, `608_2b`) and `slug` is snake_case that does not start with `r<digit>` (e.g. `r117_4_priority.rs`).",
        bad: rule_examples!("rule_file_name/bad"),
        good: rule_examples!("rule_file_name/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleFileName {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleFileName, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RuleFileName {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
