use bevy::prelude::{Commands, MessageReader};

use crate::Did;
use crate::cards;
use crate::events;

/// 509.1h An attacking creature with one or more creatures declared as blockers for it becomes a
/// blocked creature; one with no creatures declared as blockers for it becomes an unblocked
/// creature. This remains unchanged until the creature is removed from combat, an effect says that
/// it becomes blocked or unblocked, or the combat phase ends, whichever comes first. A creature
/// remains blocked even if all the creatures blocking it are removed from combat.
pub fn r509_1h_becomes_blocked(
    mut declared: MessageReader<Did<events::DeclareBlocker>>,
    mut commands: Commands,
) {
    for declared in declared.read() {
        commands.entity(declared.attacker).insert(cards::Blocked);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn declared_blockers_become_blocking_and_attacker_blocked() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats, bears_1]),
        ])?;

        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;
        let assignments = vec![
            actions::BlockAssignment {
                blocker: bears.into(),
                attacker: rats.into(),
            },
            actions::BlockAssignment {
                blocker: bears.into(),
                attacker: bears_1.into(),
            },
        ];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock {
                assignments,
                why: vec![actions::DeclarationWhy {
                    creature: Some(bears.into()),
                    why: rules::Why::BlocksMoreThanOne,
                }],
            })
        );
        assert!(!game.is_blocking(bears));
        game.play([p2.declare_blockers([(bears, rats)])])?;

        assert_eq!(
            game.get::<crate::cards::Blocking>(bears),
            Some(crate::cards::Blocking::One(rats.into()))
        );
        assert!(game.has::<crate::cards::Blocked>(rats));
        Ok(())
    }
}
