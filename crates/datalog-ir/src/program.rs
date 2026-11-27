use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::EngineError;
use crate::Rule;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deref)]
pub struct Program {
    pub rules: Vec<Rule>,
}

#[atomic_delegates]
impl Program {
    pub fn stratify(&self) -> Result<Vec<Vec<Rule>>, EngineError> {}
}

impl Program {
    #[must_use]
    pub fn canonical(mut self) -> Self {
        self.rules.sort();
        self.rules.dedup();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::Program;

    #[test]
    fn test_usage() {
        let later = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(2)), Term::Val(Value::Int(3))],
            },
            body: vec![],
        };
        let earlier = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(1)), Term::Val(Value::Int(2))],
            },
            body: vec![],
        };
        let canonical = Program {
            rules: vec![later.clone(), earlier.clone()],
        }
        .canonical();
        assert_eq!(canonical.rules, vec![earlier, later]);
    }

    #[test]
    fn canonical_is_order_independent() {
        let first = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(1)), Term::Val(Value::Int(2))],
            },
            body: vec![],
        };
        let second = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(2)), Term::Val(Value::Int(3))],
            },
            body: vec![],
        };
        let forward = Program {
            rules: vec![first.clone(), second.clone()],
        }
        .canonical();
        let backward = Program {
            rules: vec![second, first.clone(), first],
        }
        .canonical();
        assert_eq!(forward, backward);
    }
}
