use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;

pub fn accepts(choice: &actions::Choice, answer: &actions::Action) -> bool {
    let actions::Choice::May(choice) = choice else {
        return false;
    };
    let actions::Action::ChooseMay(chosen) = answer else {
        return false;
    };
    let chosen_set: BTreeSet<Entity> = chosen.iter().copied().collect();
    let options: BTreeSet<Entity> = choice.options.iter().copied().collect();
    chosen_set.len() == chosen.len() && chosen_set.is_subset(&options)
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
        let discard = actions::Choice::Discard(actions::DiscardChoice {
            player: p1.id,
            count: 1,
            legal: BTreeSet::from([first]),
        });
        assert!(!accepts(&discard, &actions::Action::ChooseMay(vec![first])));
    }
}
