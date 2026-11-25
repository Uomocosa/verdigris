use atomic_delegate_macros::atomic_delegates;

use crate::actions;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Target(actions::TargetChoice),
    May(actions::MayChoice),
    Discard(actions::DiscardChoice),
    CombatDamage(actions::CombatDamageChoice),
    OrderTriggers(actions::OrderTriggersChoice),
    ArrangeGraveyard(actions::ArrangeGraveyardChoice),
    Legendary(actions::LegendaryChoice),
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
}

impl Choice {
    #[must_use]
    pub const fn player(&self) -> Option<players::PlayerId> {
        match self {
            Self::Target(_) => None,
            Self::May(choice) => Some(choice.player),
            Self::Discard(choice) => Some(choice.player),
            Self::CombatDamage(choice) => Some(choice.player),
            Self::OrderTriggers(choice) => Some(choice.player),
            Self::ArrangeGraveyard(choice) => Some(choice.player),
            Self::Legendary(choice) => Some(choice.player),
        }
    }
}

#[atomic_delegates]
impl Choice {
    #[must_use]
    pub fn accepts(&self, answer: &actions::Action) -> bool {}
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
        assert_eq!(choice.player(), Some(p1.id));
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn target_has_no_player_while_every_other_choice_names_its_player() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let chooser = p1.id;
        let event = rules::EventId("may");
        let choices = [
            actions::Choice::Target(actions::TargetChoice {
                spell: first,
                count: 1,
                legal: BTreeSet::from([first]),
            }),
            actions::Choice::May(actions::MayChoice {
                player: chooser,
                event,
                options: vec![first],
            }),
            actions::Choice::Discard(actions::DiscardChoice {
                player: chooser,
                count: 1,
                legal: BTreeSet::from([first]),
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
        let answered_by: Vec<Option<players::PlayerId>> =
            choices.iter().map(actions::Choice::player).collect();
        assert_eq!(
            answered_by,
            [
                None,
                Some(chooser),
                Some(chooser),
                Some(chooser),
                Some(chooser),
                Some(chooser),
                Some(chooser),
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
        let discard = actions::Choice::Discard(actions::DiscardChoice {
            player: p1.id,
            count: 1,
            legal: BTreeSet::from([first]),
        });
        assert!(may.is_may_of(event));
        assert!(!may.is_may_of(rules::EventId("other")));
        assert!(!discard.is_may_of(event));
    }
}
