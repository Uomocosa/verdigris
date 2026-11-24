use crate::events;
use bevy::prelude::{Commands, MessageReader};

use crate::Resolve;
use crate::cards;

/// 701.26a To tap a permanent, turn it sideways from an upright position. Only untapped permanents can be tapped.
pub fn r701_26a_tap(mut taps: MessageReader<Resolve<events::Tap>>, mut commands: Commands) {
    for tap in taps.read() {
        commands.entity(tap.permanent).insert(cards::Tapped);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn declaring_an_attacker_taps_it() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])?;

        assert!(game.is_attacking(bears_1));
        assert!(game.is_tapped(bears_1));
        assert!(!game.is_tapped(bears_2));
        Ok(())
    }
}
