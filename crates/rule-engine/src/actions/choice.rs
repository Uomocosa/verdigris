use atomic_delegate_macros::atomic_delegates;

use crate::actions;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Target(actions::TargetChoice),
    May(actions::MayChoice),
    Pick(actions::PickChoice),
    CombatDamage(actions::CombatDamageChoice),
    OrderTriggers(actions::OrderTriggersChoice),
    ArrangeGraveyard(actions::ArrangeGraveyardChoice),
    Legendary(actions::LegendaryChoice),
    DefendingPlayer(actions::DefendingPlayerChoice),
}

#[atomic_delegates]
impl Choice {
    #[must_use]
    pub(crate) fn first_queued(queue: &actions::AskQueue) -> Option<Self> {}
}

#[rustfmt::skip]
impl Choice {
    #[must_use]
    pub fn is_may_of(&self, event: rules::EventId) -> bool { matches!(self, Self::May(choice) if choice.event == event) }

    #[must_use]
    pub const fn is_found_by_a_state_based_check(&self) -> bool { matches!(self, Self::Legendary(_)) }
}

impl Choice {
    #[must_use]
    pub const fn player(&self) -> players::PlayerId {
        match self {
            Self::Target(choice) => choice.player,
            Self::May(choice) => choice.player,
            Self::Pick(choice) => choice.player,
            Self::CombatDamage(choice) => choice.player,
            Self::OrderTriggers(choice) => choice.player,
            Self::ArrangeGraveyard(choice) => choice.player,
            Self::Legendary(choice) => choice.player,
            Self::DefendingPlayer(choice) => choice.player,
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

#[rustfmt::skip]
impl Choice {
    #[must_use]
    pub(crate) const fn blocks_priority(&self) -> bool { !matches!(self, Self::Target(_)) }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::players;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let choice = actions::Choice::May(actions::MayChoice {
            player: p1.id,
            event: rules::EventId("may"),
            options: Vec::new(),
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
            actions::Choice::Target(actions::TargetChoice {
                player: chooser,
                spell: first,
                count: 1,
                legal: BTreeSet::from([first]),
            }),
            actions::Choice::May(actions::MayChoice {
                player: chooser,
                event,
                options: vec![first],
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
            actions::Choice::OrderTriggers(actions::OrderTriggersChoice {
                player: chooser,
                triggers: vec![first],
            }),
            actions::Choice::ArrangeGraveyard(actions::ArrangeGraveyardChoice {
                player: chooser,
                graveyard: first,
                cards: vec![first],
            }),
            actions::Choice::Legendary(actions::LegendaryChoice {
                player: chooser,
                legal: BTreeSet::from([first]),
            }),
        ];
        let answered_by: Vec<players::PlayerId> =
            choices.iter().map(actions::Choice::player).collect();
        assert_eq!(
            answered_by,
            [
                chooser, chooser, chooser, chooser, chooser, chooser, chooser,
            ]
        );
    }

    #[test]
    fn only_may_choices_are_mays_of_their_event() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let event = rules::EventId("may");
        let may = actions::Choice::May(actions::MayChoice {
            player: p1.id,
            event,
            options: vec![first],
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
