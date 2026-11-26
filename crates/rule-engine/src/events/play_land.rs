use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.1/priority");
pub const FROM_HAND: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.1/from-hand");
pub const LAND_CARD: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.1/land-card");
pub const MAIN_PHASE: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.1/main-phase");
pub const EMPTY_STACK: rules::ClauseId<events::PlayLand> =
    rules::ClauseId::new("305.1/empty-stack");
pub const OWNS: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.1/owns");
pub const OWN_TURN: rules::ClauseId<events::PlayLand> = rules::ClauseId::new("305.3/own-turn");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayLand {
    pub land: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for PlayLand {
    fn subject(&self) -> Entity { self.land }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for PlayLand {
    const ID: rules::EventId = rules::EventId("305.1/play-land");

    #[atomic_delegate(PlayLand, Event)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::EMPTY_STACK;
    use super::FROM_HAND;
    use super::LAND_CARD;
    use super::MAIN_PHASE;
    use super::OWN_TURN;
    use super::OWNS;
    use super::PRIORITY;
    use super::PlayLand;

    #[test]
    fn test_usage() {
        assert_eq!(PlayLand::ID, rules::EventId("305.1/play-land"));
        let clauses = PlayLand::clauses();
        assert_eq!(clauses.len(), 7);
        assert_eq!(clauses[0].key, PRIORITY.key);
        assert_eq!(clauses[0].text, "A player who has priority");
        assert_eq!(clauses[0].role, rules::Role::Player);
        assert_eq!(
            clauses[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(clauses[1].key, LAND_CARD.key);
        assert_eq!(clauses[1].text, "a land card");
        assert_eq!(clauses[1].role, rules::Role::Subject);
        assert_eq!(
            clauses[1].condition,
            rules::card_type(cards::CardType::Land)
        );
        assert_eq!(clauses[2].key, FROM_HAND.key);
        assert_eq!(clauses[2].text, "from their hand");
        assert_eq!(clauses[2].role, rules::Role::Player);
        assert_eq!(
            clauses[2].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(clauses[3].key, MAIN_PHASE.key);
        assert_eq!(clauses[3].text, "during a main phase of their turn");
        assert_eq!(clauses[3].role, rules::Role::Player);
        assert_eq!(clauses[3].condition, rules::main_phase());
        assert_eq!(clauses[4].key, EMPTY_STACK.key);
        assert_eq!(clauses[4].text, "when the stack is empty");
        assert_eq!(clauses[4].role, rules::Role::Player);
        assert_eq!(clauses[4].condition, rules::stack_empty());
        assert_eq!(clauses[5].key, OWNS.key);
        assert_eq!(clauses[5].text, "their hand");
        assert_eq!(clauses[5].role, rules::Role::Subject);
        assert_eq!(
            clauses[5].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(clauses[6].key, OWN_TURN.key);
        assert_eq!(
            clauses[6].text,
            "A player can’t play a land, for any reason, if it isn’t their turn."
        );
        assert_eq!(clauses[6].role, rules::Role::Player);
        assert_eq!(
            clauses[6].condition,
            rules::active_player(rules::Role::Player)
        );
        assert!(clauses[6].absolute);
        assert!(
            PlayLand::clauses()
                .iter()
                .filter(|clause| clause.key != OWN_TURN.key)
                .all(|clause| !clause.absolute)
        );
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in PlayLand::clauses() {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
