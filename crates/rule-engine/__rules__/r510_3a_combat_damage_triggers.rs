use crate::events;
use bevy::prelude::{MessageReader, Query, ResMut, With};

use crate::Resolve;
use crate::abilities;
use crate::cards;
use crate::players;
use crate::stack;

/// 510.3a Any abilities that triggered on damage being dealt or while state-based actions are
/// performed afterward are put onto the stack before the active player gets priority; the order in
/// which they triggered doesn’t matter. (See rule 603, “Handling Triggered Abilities.”)
pub fn r510_3a_combat_damage_triggers(
    mut dealt: MessageReader<Resolve<events::DealDamage>>,
    mut queue: ResMut<stack::TriggerQueue>,
    sources: Query<(&cards::CardDefinition, &cards::Owner), With<cards::Attacking>>,
    targets: Query<&players::Life>,
) {
    for event in dealt.read() {
        if event.amount == 0 {
            continue;
        }
        if targets.get(event.recipient).is_err() {
            continue;
        }
        let Ok((definition, owner)) = sources.get(event.source) else {
            continue;
        };
        for ability in definition.triggered_abilities() {
            if !matches!(
                ability.event,
                abilities::TriggerEvent::DealtCombatDamageToPlayer
            ) {
                continue;
            }
            queue.push(stack::PendingTrigger {
                source: event.source,
                controller: **owner,
                ability: *ability,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn an_unblocked_thief_draws_a_card_once_its_trigger_resolves() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([SCROLL_THIEF]), P2]);
        let [thief] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SCROLL_THIEF)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([thief]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 19);
        assert!(game.trigger_sources().contains(&thief.into()));
        assert_eq!(game.player(p1).cards_in(HAND).count(), 0);

        game.play([p2.pass_priority()])?;

        assert_eq!(game.player(p1).cards_in(HAND).count(), 1);
        Ok(())
    }

    #[test]
    fn a_blocked_thief_dealing_damage_only_to_its_blocker_triggers_nothing() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SCROLL_THIEF]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [thief] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SCROLL_THIEF)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([thief]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(bears, thief)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.trigger_sources().len(), 0);
        assert_eq!(game.player(p1).cards_in(HAND).count(), 0);
        Ok(())
    }
}
