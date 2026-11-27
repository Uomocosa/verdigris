use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;

pub fn accepts(choice: &actions::Choice, answer: &actions::Action) -> bool {
    match choice {
        actions::Choice::May(choice) => {
            let actions::Action::ChooseMay(chosen) = answer else {
                return false;
            };
            let chosen_set: BTreeSet<Entity> = chosen.iter().copied().collect();
            let options: BTreeSet<Entity> = choice.options.iter().copied().collect();
            chosen_set.len() == chosen.len() && chosen_set.is_subset(&options)
        }
        actions::Choice::Pick(choice) => {
            let actions::Action::Choose(picked) = answer else {
                return false;
            };
            choice.check(picked).is_ok()
        }
        actions::Choice::DefendingPlayer(choice) => {
            let actions::Action::ChooseDefendingPlayer(chosen) = answer else {
                return false;
            };
            choice.legal.contains(chosen)
        }
        actions::Choice::ArrangeGraveyard(choice) => {
            let actions::Action::ArrangeGraveyard(answered) = answer else {
                return false;
            };
            let expected: BTreeSet<Entity> = choice.cards.iter().copied().collect();
            let received: BTreeSet<Entity> = answered.iter().copied().collect();
            received == expected && answered.len() == choice.cards.len()
        }
        actions::Choice::OrderTriggers(choice) => {
            let actions::Action::OrderTriggers(answered) = answer else {
                return false;
            };
            let expected: BTreeSet<Entity> = choice.triggers.iter().copied().collect();
            let received: BTreeSet<Entity> = answered.iter().copied().collect();
            received == expected && answered.len() == choice.triggers.len()
        }
        actions::Choice::CombatDamage(choice) => {
            let actions::Action::AssignCombatDamage(assignments) = answer else {
                return false;
            };
            choice.assigned_damage(assignments).is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::LIBRARY;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;

    use super::accepts;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::Choice::May(actions::MayChoice {
            player: p1.id,
            event: rules::EventId("may"),
            options: vec![first, second],
        });
        assert!(accepts(&choice, &actions::Action::ChooseMay(vec![first])));
        assert!(accepts(
            &choice,
            &actions::Action::ChooseMay(vec![first, second])
        ));
    }

    #[test]
    fn repeated_illegal_and_other_choice_answers_are_not_accepted() {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let outsider = game.zone_entity(p2, LIBRARY).unwrap();
        let choice = actions::Choice::May(actions::MayChoice {
            player: p1.id,
            event: rules::EventId("may"),
            options: vec![first, second],
        });
        assert!(!accepts(
            &choice,
            &actions::Action::ChooseMay(vec![first, first])
        ));
        assert!(!accepts(
            &choice,
            &actions::Action::ChooseMay(vec![outsider])
        ));
        assert!(!accepts(&choice, &actions::Action::PassPriority));
        let discard = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::exactly(1),
            ordered: false,
        };
        assert!(!accepts(
            &actions::Choice::Pick(discard.clone()),
            &actions::Action::ChooseMay(vec![first])
        ));
        assert_eq!(
            discard.check(&[first, first]),
            Err(actions::ChoiceWhy::Duplicate)
        );
        assert_eq!(
            discard.check(&[]),
            Err(actions::ChoiceWhy::TooFew { min: 1 })
        );
    }
}
