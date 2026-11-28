use crate::rules;
use frogmyr::EngineError;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TranslationError {
    #[error("condition is not a translatable legality atom: {0:?}")]
    Untranslatable(rules::Condition),
    #[error("condition index {0} exceeds the fixed holds-name table")]
    TooManyConditions(usize),
    #[error("datalog engine failed")]
    Engine(#[from] EngineError),
}
