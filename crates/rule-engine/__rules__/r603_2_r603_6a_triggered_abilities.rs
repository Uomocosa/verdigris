use bevy::prelude::{Entity, MessageReader, Query, ResMut, With};

use crate::cards;
use crate::stack;

/// 603.2. Whenever a game event or game state matches a triggered ability’s trigger event, that
/// ability automatically triggers. The ability doesn’t do anything at this point.
///
/// 603.6a Enters-the-battlefield abilities trigger when a permanent enters the battlefield. These
/// are written, “When [this object] enters, . . . “ or “Whenever a [type] enters, . . .” Each time
/// an event puts one or more permanents onto the battlefield, all permanents on the battlefield
/// (including the newcomers) are checked for any enters-the-battlefield triggers that match the
/// event.
pub fn r603_2_r603_6a_triggered_abilities(
    mut entered: MessageReader<cards::Entered>,
    mut queue: ResMut<stack::TriggerQueue>,
    entered_cards: Query<&cards::CardDefinition>,
    sources: Query<(Entity, &cards::Owner, &cards::TriggeredEnters), With<cards::OnBattlefield>>,
) {
    for event in entered.read() {
        let permanent = **event;
        let Ok(definition) = entered_cards.get(permanent) else {
            continue;
        };
        if !definition.types.contains(&cards::CardType::Creature) {
            continue;
        }
        for (source, owner, enters) in sources.iter() {
            if source == permanent {
                continue;
            }
            let ability = **enters;
            if !matches!(
                ability.event,
                cards::TriggerEvent::Enters(cards::ObjectFilter::AnotherCreature)
            ) {
                continue;
            }
            queue.push(stack::PendingTrigger {
                source,
                controller: **owner,
                ability,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::STACK;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::zones;
    use crate::{BATTLEFIELD, HAND};

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
        let stack = game.zone_entity(p1, STACK).unwrap();
        assert_eq!(
            game.app.world().get::<zones::ZoneContents>(stack).map(|contents| contents.len()),
            Some(1)
        );
        Ok(())
    }

    #[test]
    fn soul_warden_does_not_trigger_on_itself() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([SOUL_WARDEN]).mana_pool(mana![W]), P2]);
        let [warden] = game.player(p1).cards_in(HAND).exactly::<1>(SOUL_WARDEN)?;
        game.play([p1.cast(warden).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p1), 20);
        let stack = game.zone_entity(p1, STACK).unwrap();
        let stack_contents =
            game.app.world().get::<zones::ZoneContents>(stack).map_or(0, |contents| contents.len());
        assert_eq!(stack_contents, 0);
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
    #[ignore = "needs simultaneous enters: 603.6a"]
    fn soul_warden_sees_a_creature_entering_at_the_same_time() {}

    #[test]
    #[ignore = "needs a 0/0 creature card: 603.10"]
    fn creature_that_dies_on_entering_still_triggers_soul_warden() {}
}
