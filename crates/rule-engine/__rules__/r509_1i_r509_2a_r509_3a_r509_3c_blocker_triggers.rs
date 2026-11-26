use std::collections::BTreeSet;

use bevy::prelude::{Entity, MessageReader, Query, ResMut};

use crate::Did;
use crate::abilities;
use crate::cards;
use crate::events;
use crate::stack;

/// 509.1i Any abilities that trigger on blockers being declared trigger. See rule 509.2a for
/// more information.
///
/// 509.2a Any abilities that triggered on blockers being declared or that triggered during the
/// process described in rule 509.1 are put onto the stack before the active player gets
/// priority; the order in which they triggered doesn’t matter. (See rule 603, “Handling
/// Triggered Abilities.”)
///
/// 509.3a An ability that reads “Whenever [a creature] blocks, . . .” generally triggers only
/// once each combat for that creature, even if it blocks multiple creatures. It triggers if the
/// creature is declared as a blocker. It will also trigger if that creature becomes a blocker
/// as the result of an effect, but only if it wasn’t a blocking creature at that time. (See
/// rule 509.1g.) It won’t trigger if the creature is put onto the battlefield blocking.
///
/// 509.3c An ability that reads “Whenever [a creature] becomes blocked, . . .” generally
/// triggers only once each combat for that creature, even if it’s blocked by multiple
/// creatures. It will trigger if that creature becomes blocked by at least one creature
/// declared as a blocker. It will also trigger if that creature becomes blocked by an effect
/// or by a creature that’s put onto the battlefield as a blocker, but only if the attacking
/// creature was an unblocked creature at that time. (See rule 509.1h.)
pub fn r509_1i_r509_2a_r509_3a_r509_3c_blocker_triggers(
    mut declared: MessageReader<Did<events::DeclareBlocker>>,
    mut queue: ResMut<stack::TriggerQueue>,
    creatures: Query<(&cards::CardDefinition, &cards::Controller)>,
) {
    let mut fired: BTreeSet<Entity> = BTreeSet::new();
    for declared in declared.read() {
        if fired.insert(declared.blocker)
            && let Ok((definition, controller)) = creatures.get(declared.blocker)
        {
            for ability in definition.triggered_abilities() {
                if !matches!(ability.event, abilities::TriggerEvent::Blocks) {
                    continue;
                }
                queue.push(stack::PendingTrigger {
                    source: declared.blocker,
                    controller: **controller,
                    ability: ability.clone(),
                });
            }
        }
        if fired.insert(declared.attacker)
            && let Ok((definition, controller)) = creatures.get(declared.attacker)
        {
            for ability in definition.triggered_abilities() {
                if !matches!(ability.event, abilities::TriggerEvent::BecomesBlocked) {
                    continue;
                }
                queue.push(stack::PendingTrigger {
                    source: declared.attacker,
                    controller: **controller,
                    ability: ability.clone(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::pt;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_blocker_triggers_then_grows_once_the_trigger_resolves() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2.battlefield([CHUB_TOAD])]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [toad] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(CHUB_TOAD)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(toad, bears)]),
        ])?;

        assert!(game.trigger_sources().contains(&toad.into()));
        assert_eq!(game.pt(toad), Some(pt!(1, 1)));

        game.play([p2.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.pt(toad), Some(pt!(4, 4)));
        Ok(())
    }

    #[test]
    fn an_attacker_triggers_once_it_becomes_blocked() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([CHUB_TOAD]), P2.battlefield([GRIZZLY_BEARS])]);
        let [toad] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(CHUB_TOAD)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([toad]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(bears, toad)]),
        ])?;

        assert_eq!(game.trigger_sources().len(), 1);
        assert!(game.trigger_sources().contains(&toad.into()));

        game.play([p2.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.pt(toad), Some(pt!(4, 4)));
        Ok(())
    }

    #[test]
    #[ignore = "needs a card that lets a creature block an additional creature: 509.1a"]
    fn a_blocker_blocking_two_creatures_triggers_only_once() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, GRIZZLY_BEARS]),
            P2.battlefield([CHUB_TOAD]),
        ]);
        let [bears_1, bears_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(GRIZZLY_BEARS)?;
        let [toad] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(CHUB_TOAD)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1, bears_2]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(toad, bears_1), (toad, bears_2)]),
        ])?;

        assert_eq!(game.trigger_sources().len(), 1);
        assert!(game.trigger_sources().contains(&toad.into()));

        game.play([p2.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.pt(toad), Some(pt!(4, 4)));
        Ok(())
    }
}
