use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::Attempt;
use crate::GRAVEYARD;
use crate::Resolve;
use crate::cards;
use crate::damage;
use crate::zones;

/// 701.7a To create one or more tokens with certain characteristics, put the specified number of
/// tokens with the specified characteristics onto the battlefield.
///
/// # Notes
///
/// 701.7b orders replacement effects before and after continuous effects that modify a token's
/// characteristics. No replacement effect exists yet, so a token is put onto the battlefield with
/// the characteristics its creation effect defines.
pub fn r701_7a_create_tokens(
    mut requested: MessageReader<Resolve<events::CreateTokens>>,
    mut entering: MessageWriter<Attempt<events::EnterBattlefield>>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
) {
    for request in requested.read() {
        let Some(staging) = zones.find(GRAVEYARD, Some(request.controller)) else {
            continue;
        };
        for _ in 0..request.amount {
            let token = commands
                .spawn((
                    request.token.definition(),
                    zones::InZone(staging),
                    cards::Owner(request.controller),
                    damage::Damage(0),
                    cards::Token,
                ))
                .id();
            entering.write(Attempt(events::EnterBattlefield {
                permanent: token,
                controller: request.controller,
                summoning_sick: true,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn raise_the_alarm_creates_two_soldier_tokens() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([RAISE_THE_ALARM]).mana_pool(mana![1, W]), P2]);
        let [raise_the_alarm] = game.player(p1).cards_in(HAND).exactly::<1>(RAISE_THE_ALARM)?;
        game.play([p1.cast(raise_the_alarm).pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.player(p1).cards_in(BATTLEFIELD).count(), 2);
        assert_eq!(
            game.player(p1).cards_in(BATTLEFIELD).matching(SOLDIER_TOKEN).len(),
            2
        );
        assert!(game.player(p1).cards_in(GRAVEYARD).has(RAISE_THE_ALARM));
        let [soldier, _] = game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(SOLDIER_TOKEN)?;
        assert!(game.has::<cards::Token>(soldier));
        Ok(())
    }
}
