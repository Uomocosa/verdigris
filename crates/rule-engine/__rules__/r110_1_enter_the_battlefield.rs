use crate::events;
use bevy::prelude::{Commands, Has, MessageReader, MessageWriter, Query};

use crate::BATTLEFIELD;
use crate::Did;
use crate::Resolve;
use crate::cards;
use crate::zones;

type EnteringPermanent<'w, 's> = Query<
    'w,
    's,
    (
        &'static cards::CardDefinition,
        &'static cards::Controller,
        &'static cards::Owner,
        Has<cards::Token>,
        Option<&'static cards::Enchanting>,
    ),
>;

/// 110.1. A permanent is a card or token on the battlefield. A permanent remains on the battlefield
/// indefinitely. A card or token becomes a permanent as it enters the battlefield and it stops
/// being a permanent as it’s moved to another zone by an effect or rule.
pub fn r110_1_enter_the_battlefield(
    mut entering: MessageReader<Resolve<events::EnterTheBattlefield>>,
    mut entered: MessageWriter<Did<events::EnterTheBattlefield>>,
    mut commands: Commands,
    permanents: EnteringPermanent<'_, '_>,
    zones: zones::ZoneLookup,
) {
    for event in entering.read() {
        let Ok((definition, controller, owner, has_token, enchanting)) =
            permanents.get(event.permanent)
        else {
            continue;
        };
        let Some(battlefield) = zones.find(BATTLEFIELD, Some(**controller)) else {
            continue;
        };
        if has_token {
            commands.entity(event.permanent).insert((
                zones::InZone(battlefield),
                *controller,
                cards::SummoningSick,
            ));
            entered.write(Did(events::EnterTheBattlefield {
                permanent: event.permanent,
            }));
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
        commands.entity(moved).insert((*controller, cards::SummoningSick));
        if let Some(enchanting) = enchanting {
            commands.entity(moved).insert(*enchanting);
        }
        entered.write(Did(events::EnterTheBattlefield { permanent: moved }));
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
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
        assert!(game.app.world().iter_entities().all(|entity| !entity.contains::<cards::Token>()));
        Ok(())
    }
}
