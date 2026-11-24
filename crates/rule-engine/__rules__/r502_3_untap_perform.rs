use crate::events;
use bevy::prelude::{Commands, MessageReader};

use crate::Resolve;
use crate::cards;

/// 502.3. Third, the active player determines which permanents they control will untap. Then they
/// untap them all simultaneously. This turn-based action doesn’t use the stack. Normally, all of
/// a player’s permanents untap, but effects can keep one or more of a player’s permanents from
/// untapping.
pub fn r502_3_untap_perform(
    mut untaps: MessageReader<Resolve<events::Untap>>,
    mut commands: Commands,
) {
    for untap in untaps.read() {
        commands.entity(untap.permanent).remove::<cards::Tapped>();
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_permanent_that_skips_untap_stays_tapped() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([TEST_FROZEN_RELIC]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [relic] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_FROZEN_RELIC)?;
        let [p2_bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p2.pass_to(turns::Step::Upkeep)])?;

        assert!(game.is_tapped(relic));
        assert!(!game.is_tapped(p2_bears));
        Ok(())
    }
}
