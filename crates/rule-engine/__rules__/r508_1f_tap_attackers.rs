use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Added, Entity, MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Resolve;
use crate::cards;
use crate::events;

/// 508.1f The active player taps the chosen creatures. Tapping a creature when it’s declared as
/// an attacker isn’t a cost; attacking simply causes creatures to become tapped.
pub fn r508_1f_tap_attackers() -> ScheduleConfigs<ScheduleSystem> {
    (
        attempt_to_tap_attackers,
        Attempt::<events::TapAttacker>::resolved_by(tap_attacker),
    )
        .chain()
}

fn attempt_to_tap_attackers(
    attackers: Query<(Entity, &cards::Controller), Added<cards::Attacking>>,
    mut taps: MessageWriter<Attempt<events::TapAttacker>>,
) {
    for (attacker, controller) in attackers.iter() {
        taps.write(Attempt(events::TapAttacker {
            attacker,
            player: **controller,
        }));
    }
}

fn tap_attacker(
    mut attackers: MessageReader<Resolve<events::TapAttacker>>,
    mut taps: MessageWriter<Attempt<events::Tap>>,
) {
    for attacker in attackers.read() {
        taps.write(Attempt(events::Tap {
            permanent: attacker.attacker,
            controller: attacker.player,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
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
        Ok(())
    }
}
