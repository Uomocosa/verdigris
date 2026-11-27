use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::actions;
use crate::cards;
use crate::players;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ActivateManaAbility {
        source: Entity,
        produces: cards::ManaType,
    },
    Cast(Entity),
    Play(Entity),
    ChooseTargets(Vec<Entity>),
    ChooseMay(Vec<Entity>),
    ChooseDiscard(Vec<Entity>),
    ChooseLegendary(Entity),
    ChooseDefendingPlayer(players::PlayerId),
    OrderTriggers(Vec<Entity>),
    ArrangeGraveyard(Vec<Entity>),
    AssignCombatDamage(Vec<actions::DamageAssignment>),
    DeclareAttackers(Vec<Entity>),
    DeclareBlockers(Vec<actions::BlockAssignment>),
    PassPriority,
    Concede,
}

#[atomic_delegates]
impl Action {
    #[must_use]
    pub const fn answers_a_choice(&self) -> bool {}
}

impl Action {
    #[must_use]
    pub const fn rejection(&self, player: players::PlayerId) -> Option<actions::Rejected> {
        match self {
            Self::ChooseMay(_) => Some(actions::Rejected::InvalidMay { player }),
            Self::ChooseDefendingPlayer(_) => {
                Some(actions::Rejected::CannotChooseDefendingPlayer { player })
            }
            Self::ChooseDiscard(_) => Some(actions::Rejected::InvalidDiscard { player }),
            Self::ArrangeGraveyard(_) => Some(actions::Rejected::InvalidGraveyardOrder { player }),
            Self::OrderTriggers(_) => Some(actions::Rejected::InvalidTriggerOrder { player }),
            Self::AssignCombatDamage(_) => Some(actions::Rejected::InvalidCombatDamage { player }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let action = actions::Action::ChooseMay(Vec::new());
        assert_eq!(
            action.rejection(p1.id),
            Some(actions::Rejected::InvalidMay { player: p1.id })
        );
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn only_shared_choices_carry_a_rejection() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let legend = game.zone_entity(p1, BATTLEFIELD).unwrap();
        assert_eq!(
            actions::Action::ChooseDiscard(Vec::new()).rejection(p1.id),
            Some(actions::Rejected::InvalidDiscard { player: p1.id })
        );
        assert!(actions::Action::ChooseMay(Vec::new()).answers_a_choice());
        assert!(actions::Action::ChooseDiscard(Vec::new()).answers_a_choice());
        assert!(actions::Action::ChooseTargets(Vec::new()).answers_a_choice());
        assert!(actions::Action::ChooseLegendary(legend).answers_a_choice());
        assert!(!actions::Action::PassPriority.answers_a_choice());
        assert_eq!(
            actions::Action::ChooseLegendary(legend).rejection(p1.id),
            None
        );
        assert_eq!(
            actions::Action::ChooseTargets(Vec::new()).rejection(p1.id),
            None
        );
        assert_eq!(actions::Action::PassPriority.rejection(p1.id), None);
    }
}
