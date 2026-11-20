use bevy::prelude::{Commands, Entity, MessageReader, Query, With};

use crate::cards;
use crate::turns;

/// 511.3. As soon as the end of combat step ends, all creatures, battles, and planeswalkers are removed from combat. After the end of combat step ends, the combat phase is over and the postcombat main phase begins (see rule 505).
///
/// 506.4. A permanent is removed from combat if it leaves the battlefield, if its controller changes, if it phases out, if an effect specifically removes it from combat, if it’s a planeswalker that’s being attacked and stops being a planeswalker, if it’s a battle that’s being attacked and stops being a battle, or if it’s an attacking or blocking creature that regenerates (see rule 701.19), stops being a creature, or becomes a battle. A creature that’s removed from combat stops being an attacking, blocking, blocked, and/or unblocked creature. A planeswalker or battle that’s removed from combat stops being attacked.
pub fn r511_3_remove_from_combat(
    mut step_started: MessageReader<turns::StepStarted>,
    mut commands: Commands,
    attacking: Query<Entity, With<cards::Attacking>>,
    blocking: Query<Entity, With<cards::Blocking>>,
    blocked: Query<Entity, With<cards::Blocked>>,
) {
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
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        game.play([
            p1.pass_to(turns::Step::PostcombatMain),
            p2.pass_to(turns::Step::PostcombatMain),
        ])
        .unwrap();

        assert!(game.app.world().get::<crate::cards::Attacking>(bears.into()).is_none());
    }

    #[test]
    fn r511_3_blockers_are_removed_from_combat_after_end_of_combat() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();

        game.act(p1, actions::Action::DeclareAttackers(vec![rats.into()]));

        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();

        game.act(
            p2,
            actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                blocker: bears.into(),
                attacker: rats.into(),
            }]),
        );
        assert!(game.app.world().get::<crate::cards::Blocking>(bears.into()).is_some());

        game.play([
            p1.pass_to(turns::Step::PostcombatMain),
            p2.pass_to(turns::Step::PostcombatMain),
        ])
        .unwrap();

        assert!(game.app.world().get::<crate::cards::Blocking>(bears.into()).is_none());
        assert!(game.app.world().get::<crate::cards::Blocked>(rats.into()).is_none());
    }

    #[test]
    fn r511_3_attackers_are_removed_from_combat_after_end_of_combat() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        assert!(game.app.world().get::<crate::cards::Attacking>(bears.into()).is_some());

        game.play([
            p1.pass_to(turns::Step::PostcombatMain),
            p2.pass_to(turns::Step::PostcombatMain),
        ])
        .unwrap();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::PostcombatMain
        );
        assert!(game.app.world().get::<crate::cards::Attacking>(bears.into()).is_none());
    }
}
