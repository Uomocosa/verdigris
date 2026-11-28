use crate::basic::enums::ConjunctSpec;
use crate::basic::enums::TermSpec;

pub struct AtomSpec {
    pub rel: String,
    pub terms: Vec<TermSpec>,
}

pub struct RuleSpec {
    pub head: AtomSpec,
    pub body: Vec<ConjunctSpec>,
}
