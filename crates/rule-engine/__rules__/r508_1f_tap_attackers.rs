use bevy::prelude::{Added, Entity, MessageWriter, Query};

use crate::Attempt;
use crate::cards;
use crate::events;
use crate::rules::traits::event::Event;

/// 508.1f The active player taps the chosen creatures. Tapping a creature when it’s declared as
/// an attacker isn’t a cost; attacking simply causes creatures to become tapped.
pub fn r508_1f_tap_attackers(
    attackers: Query<Entity, Added<cards::Attacking>>,
    mut taps: MessageWriter<Attempt<events::Tap>>,
) {
    for attacker in attackers.iter() {
        taps.write(Attempt(events::Tap {
            permanent: attacker,
            cause: events::Attack::ID,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::Attempt;
    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules::traits::event::Event;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn vigilance_creature_does_not_tap_to_attack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([FELIDAR_SOVEREIGN]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [sovereign] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([sovereign]),
        ])?;

        assert!(game.is_attacking(sovereign));
        assert!(!game.is_tapped(sovereign));
        game.app.world_mut().write_message(Attempt(events::Tap {
            permanent: *sovereign,
            cause: events::Cast::ID,
        }));
        game.app.update();
        assert!(game.is_tapped(sovereign));
        Ok(())
    }
}
