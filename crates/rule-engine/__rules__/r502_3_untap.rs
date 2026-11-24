use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res, With};

use crate::cards;
use crate::rules;
use crate::turns;

/// 502.3. Third, the active player determines which permanents they control will untap. Then they
/// untap them all simultaneously. This turn-based action doesn’t use the stack. Normally, all of
/// a player’s permanents untap, but effects can keep one or more of a player’s permanents from
/// untapping.
pub fn r502_3_untap(
    mut step_started: MessageReader<turns::StepStarted>,
    mut untaps: MessageWriter<rules::Would<rules::Untap>>,
    turn: Res<turns::Turn>,
    tapped: Query<(Entity, &cards::Owner), With<cards::Tapped>>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::Untap {
            continue;
        }
        for (entity, owner) in tapped.iter() {
            if **owner == *turn.active {
                untaps.write(rules::Would(rules::Untap {
                    permanent: entity,
                    controller: **owner,
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn untap_step_advances_to_upkeep() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);

        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p2.pass_to(turns::Step::Upkeep)])?;

        assert_eq!(game.turn().step, turns::Step::Upkeep);
        Ok(())
    }

    #[test]
    fn the_untap_step_untaps_the_active_players_permanents_only() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([GRIZZLY_BEARS]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [p1_bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [p2_bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p2.pass_to(turns::Step::Upkeep)])?;

        assert!(!game.is_tapped(p2_bears));
        assert!(game.is_tapped(p1_bears));
        Ok(())
    }
}
