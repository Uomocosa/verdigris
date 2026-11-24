use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::GRAVEYARD;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::rules;
use crate::zones;

/// 404.1. A player’s graveyard is their discard pile. Any object that’s countered, discarded,
/// destroyed, or sacrificed is put on top of its owner’s graveyard, as is any instant or sorcery
/// spell that’s finished resolving. Each player’s graveyard starts out empty.
pub fn r404_1_put_into_graveyard(
    mut proceeds: MessageReader<Resolve<rules::PutIntoGraveyard>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    zones: zones::ZoneLookup,
) {
    for event in proceeds.read() {
        let Some(graveyard) = zones.find(GRAVEYARD, Some(event.owner)) else {
            continue;
        };
        commands.entity(event.object).insert(zones::InZone(graveyard));
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn an_effect_about_creatures_does_not_keep_an_aura() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([HOLY_STRENGTH]).mana_pool(mana![W]),
            P2.hand([MURDER])
                .mana_pool(mana![1, B, B])
                .battlefield([GRIZZLY_BEARS, TEST_NO_BURIALS]),
        ]);
        let [strength] = game.player(p1).cards_in(HAND).exactly::<1>(HOLY_STRENGTH)?;
        let [murder] = game.player(p2).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.cast(strength).targeting(bears).pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.cast(murder).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(strength));
        Ok(())
    }
}
