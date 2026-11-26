use bevy::prelude::{Entity, MessageReader, Query, ResMut, With};

use crate::Did;
use crate::abilities;
use crate::cards;
use crate::events;
use crate::stack;

/// 603.2. Whenever a game event or game state matches a triggered ability’s trigger event, that
/// ability automatically triggers. The ability doesn’t do anything at this point.
///
/// 603.6a Enters-the-battlefield abilities trigger when a permanent enters the battlefield. These
/// are written, “When [this object] enters, . . . “ or “Whenever a [type] enters, . . .” Each time
/// an event puts one or more permanents onto the battlefield, all permanents on the battlefield
/// (including the newcomers) are checked for any enters-the-battlefield triggers that match the
/// event.
///
/// # See also
///
/// 603.10. Normally, objects that exist immediately after an event are checked to see if the event
/// matched any trigger conditions, and continuous effects that exist at that time are used to
/// determine what the trigger conditions are and what the objects involved in the event look like.
/// However, some triggered abilities are exceptions to this rule; the game “looks back in time” to
/// determine if those abilities trigger, using the existence of those abilities and the appearance
/// of objects immediately prior to the event. The list of exceptions is as follows:
pub fn r603_2_r603_6a_triggered_abilities(
    mut entered: MessageReader<Did<events::EnterBattlefield>>,
    mut queue: ResMut<stack::TriggerQueue>,
    entered_cards: Query<&cards::CardDefinition>,
    sources: Query<
        (Entity, &cards::Controller, &cards::TriggeredEnters),
        With<cards::OnBattlefield>,
    >,
) {
    for event in entered.read() {
        let permanent = event.permanent;
        let Ok(definition) = entered_cards.get(permanent) else {
            continue;
        };
        if !definition.type_line.types.contains(&cards::CardType::Creature) {
            continue;
        }
        for (source, controller, enters) in sources.iter() {
            if source == permanent {
                continue;
            }
            let ability = **enters;
            if !matches!(
                ability.event,
                abilities::TriggerEvent::Enters(cards::ObjectFilter::AnotherCreature)
            ) {
                continue;
            }
            queue.push(stack::PendingTrigger {
                source,
                controller: **controller,
                ability,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn soul_warden_gains_life_when_another_creature_enters() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 21);
        Ok(())
    }

    #[test]
    fn trigger_waits_on_the_stack_before_resolving() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.stack().count(), 1);
        Ok(())
    }

    #[test]
    fn soul_warden_does_not_trigger_on_itself() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([SOUL_WARDEN]).mana_pool(mana![W]), P2]);
        let [warden] = game.player(p1).cards_in(HAND).exactly::<1>(SOUL_WARDEN)?;
        game.play([p1.cast(warden).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.stack().count(), 0);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(SOUL_WARDEN));
        Ok(())
    }

    #[test]
    fn soul_warden_triggers_for_an_opponents_creature() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1.battlefield([SOUL_WARDEN]),
                P2.active().hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            ]);
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p2.cast(bears).pass_priority(), p1.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);
        game.play([p2.pass_priority(), p1.pass_priority()])?;
        assert_eq!(game.life(p1), 21);
        assert_eq!(game.life(p2), 20);
        Ok(())
    }

    #[test]
    fn soul_warden_triggers_for_each_token_created_in_one_event() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([RAISE_THE_ALARM]).mana_pool(mana![1, W]),
            P2,
        ]);
        let [alarm] = game.player(p1).cards_in(HAND).exactly::<1>(RAISE_THE_ALARM)?;
        let [warden] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SOUL_WARDEN)?;
        game.play([p1.cast(alarm).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        game.play([p1.order_triggers([warden, warden])])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 22);
        Ok(())
    }

    #[test]
    #[ignore = "needs two permanents (one Soul Warden) put onto the battlefield in one event: 603.6a"]
    fn soul_warden_sees_a_creature_entering_at_the_same_time() {}

    #[test]
    fn creature_that_dies_on_entering_still_triggers_soul_warden() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([SOUL_WARDEN]).hand([ENDLESS_ONE]), P2]);
        let [one] = game.player(p1).cards_in(HAND).exactly::<1>(ENDLESS_ONE)?;
        game.play([p1.cast(one).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(one));
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 21);
        Ok(())
    }
}
