use std::collections::BTreeMap;

use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct RuleDocText;

impl RuleDocText {
    pub const NAME: &'static str = "rule_doc_text";
    pub const CODE: &'static str = "VRDG003";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `__rules__/` system fn's doc comment quotes the Comprehensive Rules text it implements, exactly.",
        why: "The doc comment is the rule file's specification next to its system; any drift between the quotation and the source text silently changes what the file claims to enforce.",
        fix: "Copy each implemented rule's line from the Comprehensive Rules file into the system fn's doc comment verbatim (same number, same punctuation, same quotes), list exactly the file name's rule ids, and keep cross-references under a `# See also` heading.",
        bad: rule_examples!("rule_doc_text/bad"),
        good: rule_examples!("rule_doc_text/good"),
    };
}

#[rustfmt::skip]
impl Rule for RuleDocText {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    fn namespace(&self) -> &'static str { "verdigris" }
    #[atomic_delegate(RuleDocText, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[atomic_delegates]
impl RuleDocText {
    pub fn doc_problems(
        texts: &BTreeMap<String, String>,
        ids: &[String],
        doc: &str,
    ) -> Vec<String> {
    }
}

#[rustfmt::skip]
impl RuleDocText {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
