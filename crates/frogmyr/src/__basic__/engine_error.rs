use thiserror::Error;

use crate::Rel;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EngineError {
    #[error("relation {relation:?} depends on itself through negation")]
    Unstratifiable { relation: Rel },
    #[error("variable {variable:?} in {relation:?} is not bound by a positive atom")]
    Unsafe {
        relation: Rel,
        variable: &'static str,
    },
    #[error("backend failed: {0}")]
    Backend(String),
}
