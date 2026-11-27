use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::Did;
use crate::Resolve;
use crate::cards;
use crate::events;

/// 508.1k Each chosen creature still controlled by the active player becomes an attacking creature.
/// It remains an attacking creature until it’s removed from combat or the combat phase ends,
/// whichever comes first. See rule 506.4.
pub fn r508_1k_becomes_attacking(
    mut declared: MessageReader<Resolve<events::Attack>>,
    mut did_declare: MessageWriter<Did<events::Attack>>,
    mut commands: Commands,
) {
    for declared in declared.read() {
        commands.entity(declared.attacker).insert(cards::Attacking);
        did_declare.write(Did(events::Attack {
            attacker: declared.attacker,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn declared_creatures_become_attacking() -> TestResult {
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
        assert!(!game.is_attacking(bears_2));
        Ok(())
    }
}
