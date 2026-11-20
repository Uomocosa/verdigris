use bevy::prelude::{Added, Commands, Entity, Query};

use crate::cards;

/// 508.1f The active player taps the chosen creatures. Tapping a creature when it’s declared as
/// an attacker isn’t a cost; attacking simply causes creatures to become tapped.
pub fn r508_1f_tap_attackers(
    mut commands: Commands,
    attackers: Query<Entity, Added<cards::Attacking>>,
) {
    for attacker in attackers.iter() {
        commands.entity(attacker).insert(cards::Tapped);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;

        assert!(game.app.world().get::<crate::cards::Tapped>(bears.into()).is_some());
        Ok(())
    }

    #[test]
    fn r508_1f_declaring_an_attacker_taps_it() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [attacking] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [defending] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([attacking]),
        ])?;

        assert!(game.app.world().get::<crate::cards::Attacking>(attacking.into()).is_some());
        assert!(game.app.world().get::<crate::cards::Tapped>(attacking.into()).is_some());
        assert!(game.app.world().get::<crate::cards::Tapped>(defending.into()).is_none());
        Ok(())
    }
}
