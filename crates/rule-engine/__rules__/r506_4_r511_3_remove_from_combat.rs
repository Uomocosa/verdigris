use bevy::prelude::{Commands, Entity, MessageReader, Query, With, Without};

use crate::cards;
use crate::turns;

/// 511.3. As soon as the end of combat step ends, all creatures, battles, and planeswalkers are
/// removed from combat. After the end of combat step ends, the combat phase is over and the
/// postcombat main phase begins (see rule 505).
///
/// 506.4. A permanent is removed from combat if it leaves the battlefield, if its controller
/// changes, if it phases out, if an effect specifically removes it from combat, if it’s a
/// planeswalker that’s being attacked and stops being a planeswalker, if it’s a battle that’s being
/// attacked and stops being a battle, or if it’s an attacking or blocking creature that regenerates
/// (see rule 701.19), stops being a creature, or becomes a battle. A creature that’s removed from
/// combat stops being an attacking, blocking, blocked, and/or unblocked creature. A planeswalker or
/// battle that’s removed from combat stops being attacked.
pub fn r506_4_r511_3_remove_from_combat(
    mut step_started: MessageReader<turns::StepStarted>,
    mut commands: Commands,
    attacking: Query<Entity, With<cards::Attacking>>,
    blocking: Query<Entity, With<cards::Blocking>>,
    blocked: Query<Entity, With<cards::Blocked>>,
    off_battlefield: Query<Entity, Without<cards::OnBattlefield>>,
) {
    for permanent in attacking.iter().chain(blocking.iter()).chain(blocked.iter()) {
        if off_battlefield.get(permanent).is_err() {
            continue;
        }
        commands.entity(permanent).remove::<(cards::Attacking, cards::Blocking, cards::Blocked)>();
    }
    for step in step_started.read() {
        if **step != turns::Step::PostcombatMain {
            continue;
        }
        for entity in attacking.iter() {
            commands.entity(entity).remove::<cards::Attacking>();
        }
        for entity in blocking.iter() {
            commands.entity(entity).remove::<cards::Blocking>();
        }
        for entity in blocked.iter() {
            commands.entity(entity).remove::<cards::Blocked>();
        }
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
    fn a_creature_that_leaves_the_battlefield_is_removed_from_combat() -> TestResult {
        use crate::{GRAVEYARD, HAND};

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, MOUNTAIN]).hand([LIGHTNING_BOLT]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.activate_ability(mountain.ability()).cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert!(!game.is_attacking(bears));
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
