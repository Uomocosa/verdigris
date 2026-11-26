use bevy::prelude::MessageReader;

use crate::GameMut;
use crate::Resolve;
use crate::events;

/// 500.8. Some effects can add phases to a turn. They do this by adding the phases directly after
/// the specified phase. If multiple extra phases are created after the same phase, the most
/// recently created phase will occur first.
pub fn r500_8_add_phases(mut added: MessageReader<Resolve<events::AddPhase>>, mut game: GameMut) {
    for add in added.read() {
        game.add_phase_after_this_one(add.phase);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn an_additional_beginning_phase_after_the_postcombat_main_phase_untaps_and_draws() -> TestResult
    {
        let (mut game, [p1, p2]) = MockGame::builder().turn(2, turns::Step::EndOfCombat).players([
            P1.battlefield([SPHINX_OF_THE_SECOND_SUN]).battlefield_tapped([GRIZZLY_BEARS]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let hand_before = game.player(p1).cards_in(HAND).count();
        game.play([p1.pass_to(turns::Step::End), p2.pass_to(turns::Step::End)])?;
        assert!(!game.is_tapped(bears));
        assert_eq!(game.player(p1).cards_in(HAND).count(), hand_before + 1);
        assert_eq!(game.turn().step, turns::Step::End);
        assert_eq!(game.turn().number, 2);
        Ok(())
    }
}
