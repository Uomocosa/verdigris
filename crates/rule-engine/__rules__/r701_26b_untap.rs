use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, With};

use crate::Did;
use crate::Resolve;
use crate::cards;

/// 701.26b To untap a permanent, rotate it back to the upright position from a sideways position.
/// Only tapped permanents can be untapped.
pub fn r701_26b_untap(
    mut untaps: MessageReader<Resolve<events::Untap>>,
    mut did_untaps: MessageWriter<Did<events::Untap>>,
    mut commands: Commands,
    tapped: Query<(), With<cards::Tapped>>,
) {
    for untap in untaps.read() {
        if tapped.get(untap.permanent).is_ok() {
            commands.entity(untap.permanent).remove::<cards::Tapped>();
            did_untaps.write(Did(**untap));
        }
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
            P1,
            P2.battlefield_tapped([GRIZZLY_BEARS, TEST_FROZEN_RELIC]),
        ]);
        let [relic] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(TEST_FROZEN_RELIC)?;
        let [p2_bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p2.pass_to(turns::Step::Upkeep)])?;

        assert!(game.is_tapped(relic));
        assert!(!game.is_tapped(p2_bears));
        Ok(())
    }
}
