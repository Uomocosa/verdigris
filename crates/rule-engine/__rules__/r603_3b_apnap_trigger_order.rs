use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, ResMut};

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
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<stack::TriggerQueue>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
) {
    for action in actions.read() {
        let actions::Action::OrderTriggers(answered) = &action.action else {
            continue;
        };
        let Some(actions::Choice::OrderTriggers(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidTriggerOrder {
                player: action.player,
            });
            continue;
        };
        let expected: BTreeSet<Entity> = choice.triggers.iter().copied().collect();
        let received: BTreeSet<Entity> = answered.iter().copied().collect();
        let is_chooser = action.player == choice.player;
        let is_permutation = received == expected && answered.len() == choice.triggers.len();
        if !(is_chooser && is_permutation) {
            rejected.write(actions::Rejected::InvalidTriggerOrder {
                player: action.player,
            });
            continue;
        }
        let Some(stack) = zones.find(STACK, None) else {
            continue;
        };
        let mut held: Vec<stack::PendingTrigger> = Vec::new();
        for trigger in queue.take_pending() {
            if trigger.controller == *choice.player && expected.contains(&trigger.source) {
                held.push(trigger);
            } else {
                queue.push(trigger);
            }
        }
        for source in answered {
            if let Some(position) = held.iter().position(|trigger| trigger.source == *source) {
                let trigger = held.remove(position);
                commands.spawn((
                    zones::InZone(stack),
                    cards::Owner(trigger.controller),
                    stack::TriggeredAbilityOnStack {
                        source: trigger.source,
                        controller: trigger.controller,
                        intervening_if: trigger.ability.intervening_if,
                        effect: trigger.ability.effect,
                    },
                ));
            }
        }
        for trigger in held {
            queue.push(trigger);
        }
        *pending = actions::PendingChoice(None);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::mana;
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
        game.play([p1.order_triggers([warden_1, warden_2])])?;
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
        let Some(actions::Choice::OrderTriggers(choice)) = pending else {
            panic!("expected P1 to be asked to order their triggers");
        };
        assert_eq!(choice.player, p1.id);
        let mut triggers = choice.triggers;
        triggers.sort();
        let mut expected = [Entity::from(warden_1), Entity::from(warden_2)];
        expected.sort();
        assert_eq!(triggers, expected);
        game.play([p1.order_triggers([warden_1, warden_2])])?;
        assert_eq!(
            game.trigger_sources(),
            vec![Entity::from(warden_1), Entity::from(warden_2)]
        );
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
