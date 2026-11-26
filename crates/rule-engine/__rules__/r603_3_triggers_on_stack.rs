use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{Entity, World};

use crate::actions;
use crate::cards;
use crate::players;
use crate::stack;
use crate::turns;
use crate::zones;

/// 603.3. Once an ability has triggered, its controller puts it on the stack as an object that’s
/// not a card the next time a player would receive priority. See rule 117, “Timing and Priority.”
/// The ability becomes the topmost object on the stack. It has the text of the ability that
/// created it, and no other characteristics. It remains on the stack until it’s countered, it
/// resolves, a rule causes it to be removed from the stack, or an effect moves it elsewhere.
///
/// # See also
///
/// 603.3a A triggered ability is controlled by the player who controlled its source at the time it
/// triggered, unless it’s a delayed triggered ability. To determine the controller of a delayed
/// triggered ability, see rules 603.7d–f.
///
/// 603.3b If multiple abilities have triggered since the last time a player received priority, the
/// abilities are placed on the stack in a two-part process. First, each player, in APNAP order,
/// puts each triggered ability they control with a trigger condition that isn’t another ability
/// triggering on the stack in any order they choose. (See rule 101.4.) Second, each player, in
/// APNAP order, puts all remaining triggered abilities they control on the stack in any order
/// they choose. Then the game once again checks for and performs state-based actions until none
/// are performed, then abilities that triggered during this process go on the stack. This process
/// repeats until no new state-based actions are performed and no abilities trigger. Then the
/// appropriate player gets priority.
pub fn r603_3_triggers_on_stack(world: &mut World) {
    if world.resource::<actions::PendingChoice>().is_some() {
        return;
    }
    let mut pending = world.resource_mut::<stack::TriggerQueue>().take_pending();
    if pending.is_empty() {
        return;
    }
    let active = world.resource::<turns::TurnState>().turn.active;
    let turn_order = world.resource::<turns::TurnOrder>().clone();
    let apnap = players_in_apnap_order(active, &turn_order);
    pending.sort_by_key(|trigger| apnap_position(&apnap, trigger.controller));
    let mut groups: Vec<Vec<stack::PendingTrigger>> = Vec::new();
    for trigger in pending {
        let continues_run = groups.last().is_some_and(|group| {
            group.first().is_some_and(|first| first.controller == trigger.controller)
        });
        if continues_run {
            if let Some(group) = groups.last_mut() {
                group.push(trigger);
            }
        } else {
            groups.push(vec![trigger]);
        }
    }
    let mut groups = groups.into_iter();
    while let Some(group) = groups.next() {
        if group.len() == 1 {
            let mut single = group;
            if let Some(trigger) = single.pop() {
                spawn_trigger(world, trigger);
            }
            continue;
        }
        let Some(first) = group.first() else {
            continue;
        };
        world.write_message(actions::Ask(actions::Choice::OrderTriggers(
            actions::OrderTriggersChoice {
                player: first.controller,
                triggers: group.iter().map(|trigger| trigger.source).collect(),
            },
        )));
        let _ = world.run_system_once(actions::bevy_systems::show_asks::show_asks);
        let mut queue = world.resource_mut::<stack::TriggerQueue>();
        for trigger in group {
            queue.push(trigger);
        }
        for rest in groups.by_ref() {
            for trigger in rest {
                queue.push(trigger);
            }
        }
        return;
    }
}

fn players_in_apnap_order(
    active: players::PlayerId,
    turn_order: &turns::TurnOrder,
) -> Vec<players::PlayerId> {
    let mut order = vec![active];
    let mut current = active;
    while let Some(next) = turn_order.next_after(current) {
        if next == active {
            break;
        }
        order.push(next);
        current = next;
    }
    order
}

fn apnap_position(order: &[players::PlayerId], player: players::PlayerId) -> usize {
    order.iter().position(|candidate| *candidate == player).unwrap_or(usize::MAX)
}

fn spawn_trigger(world: &mut World, trigger: stack::PendingTrigger) {
    let mut zones = world.query::<(Entity, &zones::Zone)>();
    let Some(stack) =
        zones.iter(world).find(|(_, zone)| **zone == zones::Zone::Stack).map(|(entity, _)| entity)
    else {
        return;
    };
    world.spawn((
        zones::InZone(stack),
        cards::Owner(trigger.controller),
        cards::Controller(trigger.controller),
        stack::TriggeredAbilityOnStack {
            source: trigger.source,
            controller: trigger.controller,
            intervening_if: trigger.ability.intervening_if,
            effect: trigger.ability.effect,
        },
    ));
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;

    #[test]
    fn soul_warden_trigger_goes_on_the_stack_before_priority() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.stack().count(), 1);
        assert!(game.stack().has_trigger(SOUL_WARDEN.trigger()));
        assert!(game.stack().has_no(SOUL_WARDEN));
        let [trigger] = game.stack().exactly_trigger::<1>(SOUL_WARDEN.trigger())?;
        let ability = game.get::<stack::TriggeredAbilityOnStack>(trigger).unwrap();
        assert_eq!(ability.controller, p1.id);
        assert!(!game.has::<crate::cards::CardDefinition>(trigger));
        Ok(())
    }
}
