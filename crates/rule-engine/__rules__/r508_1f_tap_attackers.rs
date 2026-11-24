use bevy::prelude::{Added, Entity, MessageWriter, Query};

use crate::Attempt;
use crate::cards;
use crate::rules;

/// 508.1f The active player taps the chosen creatures. Tapping a creature when it’s declared as
/// an attacker isn’t a cost; attacking simply causes creatures to become tapped.
pub fn r508_1f_tap_attackers(
    attackers: Query<(Entity, &cards::Owner), Added<cards::Attacking>>,
    mut taps: MessageWriter<Attempt<rules::TapAttacker>>,
) {
    for (attacker, owner) in attackers.iter() {
        taps.write(Attempt(rules::TapAttacker {
            attacker,
            player: **owner,
        }));
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
