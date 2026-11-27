use crate::actions;
use crate::players;

pub fn rejection(
    choice: &actions::Choice,
    player: players::PlayerId,
    answer: &actions::Action,
) -> Option<actions::Rejected> {
    match choice {
        actions::Choice::Pick(choice) => {
            let picked = match answer {
                actions::Action::Choose(picked) => picked.as_slice(),
                _ => &[],
            };
            match choice.check(picked) {
                Ok(()) => None,
                Err(why) => Some(actions::Rejected::InvalidChoice { player, why }),
            }
        }
        actions::Choice::CombatDamage(_) => Some(actions::Rejected::InvalidCombatDamage { player }),
        actions::Choice::ArrangeGraveyard(_) => {
            Some(actions::Rejected::InvalidGraveyardOrder { player })
        }
        actions::Choice::OrderTriggers(_) => {
            Some(actions::Rejected::InvalidTriggerOrder { player })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    use super::rejection;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let choice = actions::Choice::Pick(actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::exactly(1),
            ordered: false,
        });
        assert_eq!(
            rejection(&choice, p1.id, &actions::Action::Choose(vec![first])),
            None
        );
        assert_eq!(
            rejection(&choice, p1.id, &actions::Action::Choose(vec![first, first])),
            Some(actions::Rejected::InvalidChoice {
                player: p1.id,
                why: actions::ChoiceWhy::Duplicate,
            })
        );
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
