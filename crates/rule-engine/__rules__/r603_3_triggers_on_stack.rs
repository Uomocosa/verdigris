use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{Entity, With, World};

use crate::Game;
use crate::ORDER_TRIGGERS;
use crate::actions;
use crate::cards;
use crate::players;
use crate::stack;
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
    let pending = world.resource_mut::<stack::TriggerQueue>().take_pending();
    if !pending.is_empty() {
        let mut controllers = world.query::<&cards::Controller>();
        for trigger in pending {
            let Ok(controller) = controllers.get(world, trigger.source) else {
                continue;
            };
            world.spawn((cards::Controller(**controller), trigger));
        }
    }
    let Ok(apnap) = world.run_system_once(|game: Game| game.players_in_apnap_order()) else {
        return;
    };
    let mut held: Vec<(players::PlayerId, Entity)> = world
        .query_filtered::<(Entity, &cards::Controller), With<stack::PendingTrigger>>()
        .iter(world)
        .map(|(instance, controller)| (**controller, instance))
        .collect();
    held.sort_by_key(|(controller, _)| apnap_position(&apnap, *controller));
    let mut groups: Vec<Vec<(players::PlayerId, Entity)>> = Vec::new();
    for (controller, instance) in held {
        let continues_run = groups.last().is_some_and(|group| {
            group.first().is_some_and(|first| {
                let (first_controller, _) = first;
                first_controller == &controller
            })
        });
        if continues_run {
            if let Some(group) = groups.last_mut() {
                group.push((controller, instance));
            }
        } else {
            groups.push(vec![(controller, instance)]);
        }
    }
    for group in groups {
        if group.len() == 1 {
            let mut single = group;
            if let Some((_, instance)) = single.pop() {
                put_on_stack(world, instance);
            }
            continue;
        }
        let Some((controller, _)) = group.first() else {
            continue;
        };
        let controller = *controller;
        let among: Vec<Entity> = group.iter().map(|(_, instance)| *instance).collect();
        world.write_message(actions::Ask(actions::Choice::Pick(actions::PickChoice {
            id: ORDER_TRIGGERS,
            player: controller,
            source: None,
            among: among.iter().copied().collect(),
            count: actions::Count::exactly(u32::try_from(among.len()).unwrap_or(u32::MAX)),
            ordered: true,
        })));
        let _ = world.run_system_once(actions::bevy_systems::show_asks::show_asks);
        return;
    }
}

fn apnap_position(order: &[players::PlayerId], player: players::PlayerId) -> usize {
    order.iter().position(|candidate| *candidate == player).unwrap_or(usize::MAX)
}

fn put_on_stack(world: &mut World, instance: Entity) {
    let mut zones = world.query::<(Entity, &zones::Zone)>();
    let Some(stack) =
        zones.iter(world).find(|(_, zone)| **zone == zones::Zone::Stack).map(|(entity, _)| entity)
    else {
        return;
    };
    let Ok(mut held) = world.get_entity_mut(instance) else {
        return;
    };
    let Some(trigger) = held.get::<stack::PendingTrigger>().cloned() else {
        return;
    };
    held.remove::<stack::PendingTrigger>();
    held.insert((
        zones::InZone(stack),
        stack::TriggeredAbilityOnStack {
            source: trigger.source,
            intervening_if: trigger.ability.intervening_if,
            effect: trigger.ability.effect,
        },
    ));
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::mana;
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
        let controller = game.get::<cards::Controller>(trigger).unwrap();
        assert_eq!(*controller, p1.id);
        assert!(!game.has::<crate::cards::CardDefinition>(trigger));
        Ok(())
    }
}
