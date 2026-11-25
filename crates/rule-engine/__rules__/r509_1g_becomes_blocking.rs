use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::Did;
use crate::Resolve;
use crate::cards;
use crate::events;

/// 509.1g Each chosen creature still controlled by the defending player becomes a blocking
/// creature. Each one is blocking the attacking creatures chosen for it. It remains a blocking
/// creature until it’s removed from combat or the combat phase ends, whichever comes first. See
/// rule 506.4.
pub fn r509_1g_becomes_blocking(
    mut declared: MessageReader<Resolve<events::DeclareBlocker>>,
    mut did_declare: MessageWriter<Did<events::DeclareBlocker>>,
    mut commands: Commands,
) {
    for declared in declared.read() {
        commands.entity(declared.blocker).insert(cards::Blocking(declared.attacker));
        did_declare.write(Did(**declared));
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_creature_with_reach_can_block_a_flyer() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([WIND_DRAKE]), P2.battlefield([GIANT_SPIDER])]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([drake]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(spider, drake)])])?;

        assert_eq!(
            game.get::<crate::cards::Blocking>(spider),
            Some(crate::cards::Blocking(drake.into()))
        );
        Ok(())
    }
}
