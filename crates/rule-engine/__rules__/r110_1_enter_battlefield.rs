use crate::events;
use bevy::prelude::{Commands, Has, MessageReader, MessageWriter, Query};

use crate::BATTLEFIELD;
use crate::Did;
use crate::Resolve;
use crate::cards;
use crate::zones;

/// 110.1. A permanent is a card or token on the battlefield. A permanent remains on the battlefield
/// indefinitely. A card or token becomes a permanent as it enters the battlefield and it stops
/// being a permanent as it’s moved to another zone by an effect or rule.
pub fn r110_1_enter_battlefield(
    mut entering: MessageReader<Resolve<events::EnterBattlefield>>,
    mut entered: MessageWriter<Did<events::EnterBattlefield>>,
    mut commands: Commands,
    permanents: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        Has<cards::Token>,
        Option<&cards::Enchanting>,
    )>,
    zones: zones::ZoneLookup,
) {
    for event in entering.read() {
        let Some(battlefield) = zones.find(BATTLEFIELD, Some(event.controller)) else {
            continue;
        };
        let Ok((definition, owner, has_token, enchanting)) = permanents.get(event.permanent) else {
            continue;
        };
        if has_token {
            if event.summoning_sick {
                commands.entity(event.permanent).insert((
                    zones::InZone(battlefield),
                    cards::Controller(event.controller),
                    cards::SummoningSick,
                ));
            } else {
                commands.entity(event.permanent).insert((
                    zones::InZone(battlefield),
                    cards::Controller(event.controller),
                ));
            }
            entered.write(Did(**event));
            continue;
        }
        let moved = zones::move_object(
            &mut commands,
            event.permanent,
            definition.clone(),
            *owner,
            None,
            battlefield,
        );
        commands.entity(moved).insert(cards::Controller(event.controller));
        if let Some(enchanting) = enchanting {
            commands.entity(moved).insert(*enchanting);
        }
        if event.summoning_sick {
            commands.entity(moved).insert(cards::SummoningSick);
        }
        entered.write(Did(events::EnterBattlefield {
            permanent: moved,
            controller: event.controller,
            summoning_sick: event.summoning_sick,
        }));
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
