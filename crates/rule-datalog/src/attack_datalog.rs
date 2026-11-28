use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use frogmyr::Facts;
use frogmyr::Program;
use frogmyr::Rel;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;

use crate::TranslationError;

#[derive(Debug, Clone, Copy)]
pub struct AttackDatalog;

#[atomic_delegates]
impl AttackDatalog {
    #[must_use]
    pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {}
    #[must_use]
    pub fn attack_program() -> Result<Program, TranslationError> {}
    pub fn legality<G: frogmyr::traits::engine::Engine>(
        engine: &G,
        answers: &rules::Rules,
        attacker: Entity,
        viewer: players::PlayerId,
    ) -> Result<Option<rules::Legality>, TranslationError> {
    }
}

impl AttackDatalog {
    pub(crate) const ATTACK: &'static str = "attack";
    pub(crate) const ATTACK_CANDIDATE: Rel = Rel("attack_candidate");
    pub(crate) const HAS: Rel = Rel("has");
    pub(crate) const CREATURE: &'static str = "creature";
    pub(crate) const ON_BATTLEFIELD: &'static str = "on_battlefield";
    pub(crate) const TAPPED: &'static str = "tapped";
    pub(crate) const SUMMONING_SICK: &'static str = "summoning_sick";
}

#[cfg(test)]
mod tests {
    use frogmyr::Rel;

    use super::AttackDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(*AttackDatalog::ATTACK_CANDIDATE, "attack_candidate");
        assert_eq!(*AttackDatalog::HAS, "has");
        assert_eq!(AttackDatalog::ATTACK, "attack");
        assert_eq!(Rel(AttackDatalog::ATTACK), Rel("attack"));
        assert_eq!(AttackDatalog::CREATURE, "creature");
        assert_eq!(AttackDatalog::ON_BATTLEFIELD, "on_battlefield");
        assert_eq!(AttackDatalog::TAPPED, "tapped");
        assert_eq!(AttackDatalog::SUMMONING_SICK, "summoning_sick");
    }
}
