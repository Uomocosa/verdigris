use crate::actions;
use crate::rules;

pub fn accepts(choice: &actions::Choice, answers: &rules::Rules, answer: &actions::Action) -> bool {
    match choice {
        actions::Choice::Pick(choice) => {
            let actions::Action::Choose(picked) = answer else {
                return false;
            };
            choice.check(answers, picked).is_ok()
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

    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::LIBRARY;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;

    use super::accepts;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([first, second])),
            count: actions::Count::up_to(2),
            ordered: false,
        });
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(accepts(
                    &choice,
                    &answers,
                    &actions::Action::Choose(vec![first])
                ));
                assert!(accepts(
                    &choice,
                    &answers,
                    &actions::Action::Choose(vec![first, second])
                ));
            })
            .unwrap();
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn repeated_illegal_and_other_choice_answers_are_not_accepted() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let outsider = game.zone_entity(p2, LIBRARY).unwrap();
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([first, second])),
            count: actions::Count::up_to(2),
            ordered: false,
        });
        let discard = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([first])),
            count: actions::Count::exactly(1),
            ordered: false,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!accepts(
                    &choice,
                    &answers,
                    &actions::Action::Choose(vec![first, first])
                ));
                assert!(!accepts(
                    &choice,
                    &answers,
                    &actions::Action::Choose(vec![outsider])
                ));
                assert!(!accepts(&choice, &answers, &actions::Action::PassPriority));
                assert!(!accepts(
                    &actions::Choice::Pick(discard.clone()),
                    &answers,
                    &actions::Action::PassPriority
                ));
                assert_eq!(
                    discard.check(&answers, &[first, first]),
                    Err(actions::ChoiceWhy::Duplicate)
                );
                assert_eq!(
                    discard.check(&answers, &[]),
                    Err(actions::ChoiceWhy::TooFew { min: 1 })
                );
            })
            .unwrap();
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
