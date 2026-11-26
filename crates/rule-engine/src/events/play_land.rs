use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::RestrictionId = rules::RestrictionId("305.1/priority");
pub const FROM_HAND: rules::RestrictionId = rules::RestrictionId("305.1/from-hand");
pub const LAND_CARD: rules::RestrictionId = rules::RestrictionId("305.1/land-card");
pub const MAIN_PHASE: rules::RestrictionId = rules::RestrictionId("305.1/main-phase");
pub const EMPTY_STACK: rules::RestrictionId = rules::RestrictionId("305.1/empty-stack");
pub const OWNS: rules::RestrictionId = rules::RestrictionId("305.1/owns");
pub const OWN_TURN: rules::RestrictionId = rules::RestrictionId("305.3/own-turn");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct PlayLand {
    #[arg]
    pub land: Entity,
    #[arg]
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
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

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
        let restrictions = PlayLand::restrictions();
        assert_eq!(restrictions.len(), 7);
        assert_eq!(restrictions[0].id, PRIORITY);
        assert_eq!(restrictions[0].text, "A player who has priority");
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(PlayLand::PLAYER)
        );
        assert_eq!(restrictions[1].id, LAND_CARD);
        assert_eq!(restrictions[1].text, "a land card");
        assert_eq!(
            restrictions[1].condition,
            rules::card_type(cards::CardType::Land).of(PlayLand::LAND)
        );
        assert_eq!(restrictions[2].id, FROM_HAND);
        assert_eq!(restrictions[2].text, "from their hand");
        assert_eq!(
            restrictions[2].condition,
            rules::in_zone(HAND, PlayLand::PLAYER).of(PlayLand::LAND)
        );
        assert_eq!(restrictions[3].id, MAIN_PHASE);
        assert_eq!(restrictions[3].text, "during a main phase of their turn");
        assert_eq!(restrictions[3].condition, rules::main_phase());
        assert_eq!(restrictions[4].id, EMPTY_STACK);
        assert_eq!(restrictions[4].text, "when the stack is empty");
        assert_eq!(restrictions[4].condition, rules::stack_empty());
        assert_eq!(restrictions[5].id, OWNS);
        assert_eq!(restrictions[5].text, "their hand");
        assert_eq!(
            restrictions[5].condition,
            rules::points_to::<cards::Owner>(PlayLand::PLAYER).of(PlayLand::LAND)
        );
        assert_eq!(restrictions[6].id, OWN_TURN);
        assert_eq!(
            restrictions[6].text,
            "A player can’t play a land, for any reason, if it isn’t their turn."
        );
        assert_eq!(
            restrictions[6].condition,
            rules::active_player(PlayLand::PLAYER)
        );
        assert!(restrictions[6].unwaivable);
        assert!(
            PlayLand::restrictions()
                .iter()
                .filter(|restriction| restriction.id != OWN_TURN)
                .all(|restriction| !restriction.unwaivable)
        );
        assert_eq!(PlayLand::args(), &[PlayLand::LAND, PlayLand::PLAYER,]);
    }

    #[test]
    fn restriction_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for restriction in PlayLand::restrictions() {
            assert!(
                RULES_TEXT.contains(restriction.text),
                "restriction {:?} text not found verbatim in Comprehensive Rules: {:?}",
                restriction.id,
                restriction.text
            );
        }
    }
}
