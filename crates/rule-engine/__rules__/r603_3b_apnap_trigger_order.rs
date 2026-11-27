use bevy::prelude::{Commands, Entity, MessageReader, Query, ResMut};

use crate::ORDER_TRIGGERS;
use crate::STACK;
use crate::actions;
use crate::cards;
use crate::stack;
use crate::zones;

/// 603.3b If multiple abilities have triggered since the last time a player received priority, the
/// abilities are placed on the stack in a two-part process. First, each player, in APNAP order,
/// puts each triggered ability they control with a trigger condition that isn’t another ability
/// triggering on the stack in any order they choose. (See rule 101.4.) Second, each player, in
/// APNAP order, puts all remaining triggered abilities they control on the stack in any order
/// they choose. Then the game once again checks for and performs state-based actions until none
/// are performed, then abilities that triggered during this process go on the stack. This process
/// repeats until no new state-based actions are performed and no abilities trigger. Then the
/// appropriate player gets priority.
///
/// # See also
///
/// 101.4. If multiple players would make choices and/or take actions at the same time, the active
/// player (the player whose turn it is) makes any choices required, then the next player in turn
/// order (usually the player seated to the active player’s left) makes any choices required,
/// followed by the remaining nonactive players in turn order. Then the actions happen
/// simultaneously. This rule is often referred to as the “Active Player, Nonactive Player (APNAP)
/// order” rule.
pub fn r603_3b_apnap_trigger_order(
    mut answers: MessageReader<actions::Answered>,
    mut queue: ResMut<stack::TriggerQueue>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
    controllers: Query<&cards::Controller>,
    held: Query<&stack::PendingTrigger>,
) {
    let mut ordered: Vec<Entity> = Vec::new();
    let mut answered = false;
    for answer in answers.read() {
        let actions::Choice::Pick(choice) = &answer.choice else {
            continue;
        };
        if choice.id != ORDER_TRIGGERS {
            continue;
        }
        let actions::Action::Choose(instances) = &answer.answer else {
            continue;
        };
        answered = true;
        ordered.extend(instances.iter().copied());
    }
    if !answered {
        return;
    }
    let Some(stack) = zones.find(STACK, None) else {
        return;
    };
    for instance in ordered {
        let Ok(trigger) = held.get(instance) else {
            continue;
        };
        let trigger = trigger.clone();
        commands.entity(instance).remove::<stack::PendingTrigger>();
        commands.entity(instance).insert((
            zones::InZone(stack),
            stack::TriggeredAbilityOnStack {
                source: trigger.source,
                intervening_if: trigger.ability.intervening_if,
                effect: trigger.ability.effect,
            },
        ));
    }
    for trigger in queue.take_pending() {
        let Ok(controller) = controllers.get(trigger.source) else {
            continue;
        };
        commands.spawn((cards::Controller(**controller), trigger));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::ORDER_TRIGGERS;
    use crate::actions;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn two_soul_wardens_put_two_triggers_on_the_stack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN, SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [warden_1, warden_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(SOUL_WARDEN)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.trigger_sources().len(), 0);
        let Some(actions::Choice::Pick(choice)) =
            game.app.world().resource::<actions::PendingChoice>().as_ref().cloned()
        else {
            panic!("expected P1 to be asked to order their triggers");
        };
        assert_eq!(choice.id, ORDER_TRIGGERS);
        let instances: Vec<Entity> = choice.fixed().unwrap().iter().copied().collect();
        assert_eq!(instances.len(), 2);
        let mut sources: Vec<Entity> = instances
            .iter()
            .map(|instance| game.get::<stack::PendingTrigger>(*instance).unwrap().source)
            .collect();
        sources.sort();
        let mut expected = [Entity::from(warden_1), Entity::from(warden_2)];
        expected.sort();
        assert_eq!(sources, expected);
        game.play([p1.order_triggers([instances[0], instances[1]])])?;
        assert_eq!(game.trigger_sources().len(), 2);
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 22);
        Ok(())
    }

    #[test]
    fn controller_chooses_the_order_of_their_triggers() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN, SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [warden_1, warden_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(SOUL_WARDEN)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        let pending = game.app.world().resource::<actions::PendingChoice>().as_ref().cloned();
        let Some(actions::Choice::Pick(choice)) = pending else {
            panic!("expected P1 to be asked to order their triggers");
        };
        assert_eq!(choice.id, ORDER_TRIGGERS);
        assert_eq!(choice.player, p1.id);
        let mut instances: Vec<Entity> = choice.fixed().unwrap().iter().copied().collect();
        assert_eq!(instances.len(), 2);
        let mut sources: Vec<Entity> = instances
            .iter()
            .map(|instance| game.get::<stack::PendingTrigger>(*instance).unwrap().source)
            .collect();
        sources.sort();
        let mut expected = [Entity::from(warden_1), Entity::from(warden_2)];
        expected.sort();
        assert_eq!(sources, expected);
        instances.reverse();
        let ordered: Vec<Entity> = instances
            .iter()
            .map(|instance| game.get::<stack::PendingTrigger>(*instance).unwrap().source)
            .collect();
        game.play([p1.order_triggers([instances[0], instances[1]])])?;
        assert_eq!(game.trigger_sources(), ordered);
        Ok(())
    }

    #[test]
    fn active_players_triggers_go_on_the_stack_first() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2.battlefield([SOUL_WARDEN]),
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [p1_warden] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SOUL_WARDEN)?;
        let [p2_warden] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(SOUL_WARDEN)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(
            game.trigger_sources(),
            vec![Entity::from(p1_warden), Entity::from(p2_warden)]
        );
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p2), 21);
        assert_eq!(game.life(p1), 20);
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 21);
        assert_eq!(game.life(p2), 21);
        Ok(())
    }
}
