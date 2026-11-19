use crate::actions;
use crate::testing;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PlayError {
    #[error("step {index} ({step:?}) was rejected: {rejection:?}")]
    Rejected {
        index: usize,
        step: testing::PlayStep,
        rejection: actions::Rejected,
    },
}
