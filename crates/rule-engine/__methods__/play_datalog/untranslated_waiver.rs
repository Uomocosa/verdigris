use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Atom;

use crate::legality;

pub fn untranslated_waiver<E: Event>(
    answers: &rules::Rules,
    event: &E,
    id: rules::RestrictionId,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    candidate: &Atom,
) -> bool {
    legality::PlayDatalog::holder_changes(answers, event, rules::Target::Restriction(id))
        .iter()
        .any(|change| match change {
            rules::Change::Allow { when, .. } => {
                let bound = when.bind(event);
                let mut scratch = Vec::new();
                let mut scratch_next = 0;
                legality::PlayDatalog::translate_holds(
                    &bound,
                    card,
                    holder,
                    legality::PlayDatalog::CARD_VAR,
                    candidate,
                    &mut scratch,
                    &mut scratch_next,
                )
                .is_err()
                    && answers.eval(&bound, event, None)
            }
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::events;
    use crate::players;
    use crate::rules;
    use crate::testing::FOREST;
    use crate::testing::GARRUKS_COMPANION;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;

    use super::untranslated_waiver;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([FOREST]).battlefield([GARRUKS_COMPANION]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        let [companion] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION).unwrap();
        game.read_rules(|answers, _legal| {
            let play = events::Play {
                card: *forest,
                player: players::PlayerId(*p1),
            };
            assert!(!untranslated_waiver(
                answers,
                &play,
                events::play::FROM_HAND,
                &rules::ArgRef::Arg(events::Play::CARD),
                &rules::ArgRef::Arg(events::Play::PLAYER),
                &testing::play_candidate(),
            ));
            let strike = events::AssignCombatDamage {
                target: *p2,
                source: *companion,
                amount: 3,
            };
            assert!(untranslated_waiver(
                answers,
                &strike,
                events::assign_combat_damage::ONLY_BLOCKERS,
                &rules::ArgRef::Arg(events::AssignCombatDamage::SOURCE),
                &rules::ArgRef::Arg(events::AssignCombatDamage::SOURCE),
                &testing::play_candidate(),
            ));
        })
        .unwrap();
    }
}
