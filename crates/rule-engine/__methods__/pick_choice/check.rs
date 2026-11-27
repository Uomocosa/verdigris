use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;
use crate::rules;

pub fn check(
    choice: &actions::PickChoice,
    answers: &rules::Rules,
    picked: &[Entity],
) -> Result<(), actions::ChoiceWhy> {
    let distinct: BTreeSet<Entity> = picked.iter().copied().collect();
    if distinct.len() != picked.len() {
        return Err(actions::ChoiceWhy::Duplicate);
    }
    let legal: BTreeSet<Entity> = choice
        .candidates(answers)
        .into_iter()
        .filter(|(_, legality)| *legality == rules::Legality::Legal)
        .map(|(entity, _)| entity)
        .collect();
    if let Some(offender) = picked.iter().find(|card| !legal.contains(card)) {
        return Err(actions::ChoiceWhy::NotAllowed(*offender));
    }
    let count = u32::try_from(picked.len()).unwrap_or(u32::MAX);
    if count < choice.count.min {
        return Err(actions::ChoiceWhy::TooFew {
            min: choice.count.min,
        });
    }
    if count > choice.count.max {
        return Err(actions::ChoiceWhy::TooMany {
            max: choice.count.max,
        });
    }
    if choice.ordered && picked.len() != legal.len() {
        return Err(actions::ChoiceWhy::MustOrderAll);
    }
    Ok(())
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

    use super::check;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::PickChoice {
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
                assert_eq!(check(&choice, &answers, &[first]), Ok(()));
            })
            .unwrap();
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn illegal_picks_name_their_reason() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let second = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let outsider = game.zone_entity(p2, LIBRARY).unwrap();
        let choice = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([first, second])),
            count: actions::Count::exactly(2),
            ordered: false,
        };
        let ordered = actions::PickChoice {
            ordered: true,
            ..choice.clone()
        };
        let up_to_one = actions::PickChoice {
            count: actions::Count::up_to(1),
            ..choice.clone()
        };
        let ordered_pair = actions::PickChoice {
            count: actions::Count::up_to(2),
            ordered: true,
            ..choice.clone()
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    check(&choice, &answers, &[first, first]),
                    Err(actions::ChoiceWhy::Duplicate)
                );
                assert_eq!(
                    check(&choice, &answers, &[first, outsider]),
                    Err(actions::ChoiceWhy::NotAllowed(outsider))
                );
                assert_eq!(
                    check(&choice, &answers, &[first]),
                    Err(actions::ChoiceWhy::TooFew { min: 2 })
                );
                assert_eq!(
                    check(&ordered, &answers, &[first, second, outsider]),
                    Err(actions::ChoiceWhy::NotAllowed(outsider))
                );
                assert_eq!(
                    check(&up_to_one, &answers, &[first, second]),
                    Err(actions::ChoiceWhy::TooMany { max: 1 })
                );
                assert_eq!(
                    check(&ordered_pair, &answers, &[first]),
                    Err(actions::ChoiceWhy::MustOrderAll)
                );
            })
            .unwrap();
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
