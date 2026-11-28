use crate::players;
use crate::rules;
use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use frogmyr::Facts;
use frogmyr::Program;
use frogmyr::Rel;

use crate::events;
use crate::legality;

#[derive(Debug, Clone, Copy)]
pub struct BlockDatalog;

#[atomic_delegates]
impl BlockDatalog {
    #[must_use]
    pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {}
    #[must_use]
    pub fn block_program(
        answers: &rules::Rules,
        event: &events::DeclareBlocker,
    ) -> Result<Program, legality::TranslationError> {
    }
    pub fn legality(
        answers: &rules::Rules,
        blocker: Entity,
        attacker: Entity,
        viewer: players::PlayerId,
    ) -> Result<Option<rules::Legality>, legality::TranslationError> {
    }
}

impl BlockDatalog {
    pub(crate) const BLOCK: &'static str = "block";
    pub(crate) const BLOCK_CANDIDATE: Rel = Rel("block_candidate");
}

#[cfg(test)]
mod tests {
    use frogmyr::Rel;

    use super::BlockDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(*BlockDatalog::BLOCK_CANDIDATE, "block_candidate");
        assert_eq!(BlockDatalog::BLOCK, "block");
        assert_eq!(Rel(BlockDatalog::BLOCK), Rel("block"));
    }
}
