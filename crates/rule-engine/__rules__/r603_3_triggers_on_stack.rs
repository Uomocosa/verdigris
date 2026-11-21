use bevy::prelude::World;

use crate::cards;
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
pub fn r603_3_triggers_on_stack(world: &mut World) {
    let pending = world.resource_mut::<stack::TriggerQueue>().take_pending();
    if pending.is_empty() {
        return;
    }
    let mut zones = world.query::<(bevy::prelude::Entity, &zones::Zone)>();
    let Some(stack) =
        zones.iter(world).find(|(_, zone)| **zone == zones::Zone::Stack).map(|(entity, _)| entity)
    else {
        return;
    };
    for trigger in pending {
        world.spawn((
            zones::InZone(stack),
            cards::Owner(trigger.controller),
            stack::TriggeredAbilityOnStack {
                source: trigger.source,
                controller: trigger.controller,
                effect: trigger.ability.effect,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::STACK;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn soul_warden_trigger_goes_on_the_stack_before_priority() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        let world = game.app.world();
        let stack = game.zone_entity(p1, STACK).unwrap();
        assert_eq!(
            world.get::<zones::ZoneContents>(stack).map(|contents| contents.len()),
            Some(1)
        );
        let trigger = world
            .iter_entities()
            .find_map(|entity| {
                entity
                    .get::<stack::TriggeredAbilityOnStack>()
                    .map(|ability| (entity.id(), *ability))
            })
            .unwrap();
        assert_eq!(trigger.1.controller, *p1);
        assert!(world.get::<crate::cards::CardDefinition>(trigger.0).is_none());
        Ok(())
    }
}
