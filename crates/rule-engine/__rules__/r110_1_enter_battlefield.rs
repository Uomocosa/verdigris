use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::BATTLEFIELD;
use crate::Resolve;
use crate::cards;
use crate::zones;

/// 110.1. A permanent is a card or token on the battlefield. A permanent remains on the battlefield
/// indefinitely. A card or token becomes a permanent as it enters the battlefield and it stops
/// being a permanent as it’s moved to another zone by an effect or rule.
pub fn r110_1_enter_battlefield(
    mut entering: MessageReader<Resolve<events::EnterBattlefield>>,
    mut entered: MessageWriter<cards::Entered>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
) {
    for event in entering.read() {
        let Some(battlefield) = zones.find(BATTLEFIELD, Some(event.controller)) else {
            continue;
        };
        if event.summoning_sick {
            commands
                .entity(event.permanent)
                .insert((zones::InZone(battlefield), cards::SummoningSick));
        } else {
            commands.entity(event.permanent).insert(zones::InZone(battlefield));
        }
        entered.write(cards::Entered(event.permanent));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn no_tokens_stops_raise_the_alarm_from_leaving_any_soldier_on_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([RAISE_THE_ALARM]).battlefield([TEST_NO_TOKENS]).mana_pool(mana![1, W]),
            P2,
        ]);
        let [raise_the_alarm] = game.player(p1).cards_in(HAND).exactly::<1>(RAISE_THE_ALARM)?;

        game.play([p1.cast(raise_the_alarm).pass_priority(), p2.pass_priority()])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(SOLDIER_TOKEN));
        assert_eq!(game.player(p1).cards_in(BATTLEFIELD).count(), 1);
        Ok(())
    }
}
