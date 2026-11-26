use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, Query};

use crate::cards;
use crate::turns;

/// 511.3. As soon as the end of combat step ends, all creatures, battles, and planeswalkers are
/// removed from combat. After the end of combat step ends, the combat phase is over and the
/// postcombat main phase begins (see rule 505).
pub fn r511_3_end_of_combat_ends() -> ScheduleConfigs<ScheduleSystem> {
    remove_all_from_combat.run_if(turns::Step::PostcombatMain.begins())
}

fn remove_all_from_combat(mut commands: Commands, in_combat: Query<Entity, cards::InCombat>) {
    for permanent in &in_combat {
        commands.entity(permanent).remove::<(
            cards::Attacking,
            cards::Blocking,
            cards::Blocked,
            cards::HadFirstStrikeAsDamageBegan,
        )>();
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn blockers_are_removed_from_combat_after_end_of_combat() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;

        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, rats)])])?;
        assert!(game.is_blocking(bears));

        game.play([
            p1.pass_to(turns::Step::PostcombatMain),
            p2.pass_to(turns::Step::PostcombatMain),
        ])?;

        assert!(!game.is_blocking(bears));
        assert!(!game.has::<crate::cards::Blocked>(rats));
        Ok(())
    }

    #[test]
    fn attackers_are_removed_from_combat_after_end_of_combat() -> TestResult {
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
        assert!(game.is_attacking(bears));

        game.play([
            p1.pass_to(turns::Step::PostcombatMain),
            p2.pass_to(turns::Step::PostcombatMain),
        ])?;

        assert_eq!(game.turn().step, turns::Step::PostcombatMain);
        assert!(!game.is_attacking(bears));
        Ok(())
    }
}
