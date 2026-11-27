use atomic_delegate_macros::atomic_delegates;

use crate::LEGEND;
use crate::TARGET;
use crate::actions;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Pick(actions::PickChoice),
    CombatDamage(actions::CombatDamageChoice),
}

#[atomic_delegates]
impl Choice {
    #[must_use]
    pub(crate) fn first_queued(queue: &actions::AskQueue) -> Option<Self> {}
}

#[rustfmt::skip]
impl Choice {
    #[must_use]
    pub fn is_may_of(&self, event: rules::EventId) -> bool { matches!(self, Self::Pick(choice) if *choice.id == *event) }
}

impl Choice {
    #[must_use]
    pub fn is_found_by_a_state_based_check(&self) -> bool {
        match self {
            Self::Pick(choice) => choice.id == LEGEND,
            Self::CombatDamage(_) => false,
        }
    }

    #[must_use]
    pub const fn player(&self) -> players::PlayerId {
        match self {
            Self::Pick(choice) => choice.player,
            Self::CombatDamage(choice) => choice.player,
        }
    }
}

#[atomic_delegates]
impl Choice {
    #[must_use]
    pub fn accepts(&self, answer: &actions::Action) -> bool {}

    #[must_use]
    pub fn rejection(
        &self,
        player: players::PlayerId,
        answer: &actions::Action,
    ) -> Option<actions::Rejected> {
    }
}

impl Choice {
    #[must_use]
    pub(crate) fn blocks_priority(&self) -> bool {
        match self {
            Self::Pick(choice) => choice.id != TARGET,
            Self::CombatDamage(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::LEGEND;
    use crate::ORDER_TRIGGERS;
    use crate::TARGET;
    use crate::actions;
    use crate::players;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: BTreeSet::new(),
            count: actions::Count::up_to(0),
            ordered: false,
        });
        assert_eq!(choice.player(), p1.id);
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn every_choice_names_its_player() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let chooser = p1.id;
        let event = rules::EventId("may");
        let choices = [
            actions::Choice::Pick(actions::PickChoice {
                id: TARGET,
                player: chooser,
                source: Some(first),
                among: BTreeSet::from([first]),
                count: actions::Count::exactly(1),
                ordered: false,
            }),
            actions::Choice::Pick(actions::PickChoice {
                id: actions::ChoiceId(*event),
                player: chooser,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::up_to(1),
                ordered: false,
            }),
            actions::Choice::Pick(actions::PickChoice {
                id: actions::ChoiceId("701.9b/discard"),
                player: chooser,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::exactly(1),
                ordered: false,
            }),
            actions::Choice::CombatDamage(actions::CombatDamageChoice {
                player: chooser,
                needed: Vec::new(),
            }),
            actions::Choice::Pick(actions::PickChoice {
                id: ORDER_TRIGGERS,
                player: chooser,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::exactly(1),
                ordered: true,
            }),
            actions::Choice::Pick(actions::PickChoice {
                id: LEGEND,
                player: chooser,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::exactly(1),
                ordered: false,
            }),
        ];
        let answered_by: Vec<players::PlayerId> =
            choices.iter().map(actions::Choice::player).collect();
        assert_eq!(
            answered_by,
            [chooser, chooser, chooser, chooser, chooser, chooser,]
        );
    }

    #[test]
    fn only_may_choices_are_mays_of_their_event() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let event = rules::EventId("may");
        let may = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId(*event),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::up_to(1),
            ordered: false,
        });
        let discard = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::exactly(1),
            ordered: false,
        });
        assert!(may.is_may_of(event));
        assert!(!may.is_may_of(rules::EventId("other")));
        assert!(!discard.is_may_of(event));
    }
}
