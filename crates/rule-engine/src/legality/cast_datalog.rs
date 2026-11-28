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
pub struct CastDatalog;

#[atomic_delegates]
impl CastDatalog {
    #[must_use]
    pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {}
    #[must_use]
    pub fn cast_program(
        answers: &rules::Rules,
        event: &events::Cast,
    ) -> Result<Program, legality::TranslationError> {
    }
    pub fn legality(
        answers: &rules::Rules,
        spell: Entity,
        caster: players::PlayerId,
    ) -> Result<Option<rules::Legality>, legality::TranslationError> {
    }
}

impl CastDatalog {
    pub(crate) const CAST: &'static str = "cast";
    pub(crate) const CAST_CANDIDATE: Rel = Rel("cast_candidate");
}

#[cfg(test)]
mod tests {
    use frogmyr::Rel;

    use super::CastDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(*CastDatalog::CAST_CANDIDATE, "cast_candidate");
        assert_eq!(CastDatalog::CAST, "cast");
        assert_eq!(Rel(CastDatalog::CAST), Rel("cast"));
    }
}
