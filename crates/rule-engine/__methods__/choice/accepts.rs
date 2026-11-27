use crate::actions;

pub fn accepts(choice: &actions::Choice, answer: &actions::Action) -> bool {
    match choice {
        actions::Choice::Pick(choice) => {
            let actions::Action::Choose(picked) = answer else {
                return false;
            };
            choice.check(picked).is_ok()
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
    use crate::testing::*;

    use super::accepts;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first, second]),
            count: actions::Count::up_to(2),
            ordered: false,
        });
        assert!(accepts(&choice, &actions::Action::Choose(vec![first])));
        assert!(accepts(
            &choice,
            &actions::Action::Choose(vec![first, second])
        ));
    }

    #[test]
    fn repeated_illegal_and_other_choice_answers_are_not_accepted() {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let outsider = game.zone_entity(p2, LIBRARY).unwrap();
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first, second]),
            count: actions::Count::up_to(2),
            ordered: false,
        });
        assert!(!accepts(
            &choice,
            &actions::Action::Choose(vec![first, first])
        ));
        assert!(!accepts(&choice, &actions::Action::Choose(vec![outsider])));
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
            &actions::Action::PassPriority
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
