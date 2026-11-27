use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;

pub fn check(choice: &actions::PickChoice, picked: &[Entity]) -> Result<(), actions::ChoiceWhy> {
    let distinct: BTreeSet<Entity> = picked.iter().copied().collect();
    if distinct.len() != picked.len() {
        return Err(actions::ChoiceWhy::Duplicate);
    }
    if let Some(offender) = picked.iter().find(|card| !choice.among.contains(card)) {
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
    if choice.ordered && picked.len() != choice.among.len() {
        return Err(actions::ChoiceWhy::MustOrderAll);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::LIBRARY;
    use crate::actions;
    use crate::testing::*;

    use super::check;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::exactly(1),
            ordered: false,
        };
        assert_eq!(check(&choice, &[first]), Ok(()));
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }

    #[test]
    fn illegal_picks_name_their_reason() {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let second = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let outsider = game.zone_entity(p2, LIBRARY).unwrap();
        let choice = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first, second]),
            count: actions::Count::exactly(2),
            ordered: false,
        };
        assert_eq!(
            check(&choice, &[first, first]),
            Err(actions::ChoiceWhy::Duplicate)
        );
        assert_eq!(
            check(&choice, &[first, outsider]),
            Err(actions::ChoiceWhy::NotAllowed(outsider))
        );
        assert_eq!(
            check(&choice, &[first]),
            Err(actions::ChoiceWhy::TooFew { min: 2 })
        );
        let ordered = actions::PickChoice {
            ordered: true,
            ..choice.clone()
        };
        assert_eq!(
            check(&ordered, &[first, second, outsider]),
            Err(actions::ChoiceWhy::NotAllowed(outsider))
        );
        let up_to_one = actions::PickChoice {
            count: actions::Count::up_to(1),
            ..choice.clone()
        };
        assert_eq!(
            check(&up_to_one, &[first, second]),
            Err(actions::ChoiceWhy::TooMany { max: 1 })
        );
        let ordered_pair = actions::PickChoice {
            count: actions::Count::up_to(2),
            ordered: true,
            ..choice
        };
        assert_eq!(
            check(&ordered_pair, &[first]),
            Err(actions::ChoiceWhy::MustOrderAll)
        );
    }
}
