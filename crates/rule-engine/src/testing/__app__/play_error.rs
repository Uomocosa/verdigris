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
    #[error("game stalled: too many passes without progress")]
    Stalled,
}

impl PlayError {
    #[must_use]
    pub const fn rejection(&self) -> Option<&actions::Rejected> {
        match self {
            Self::Rejected { rejection, .. } => Some(rejection),
            Self::Stalled => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayError;

    #[test]
    fn test_usage() {
        assert!(PlayError::Stalled.rejection().is_none());
    }
}
