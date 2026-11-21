use crate::GRAVEYARD;
use bevy::prelude::{Commands, MessageReader, Query, ResMut, With, Without};

use crate::StateBasedActionsPerformed;
use crate::cards;
use crate::damage;
use crate::zones;

/// 702.12b A permanent with indestructible can’t be destroyed. Such permanents aren’t destroyed by
/// lethal damage, and they ignore the state-based action that checks for lethal damage (see rule
/// 704.5g).
pub fn r702_12b_indestructible(
    mut destructions: MessageReader<damage::Destruction>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    cards: Query<&cards::Owner, (With<cards::OnBattlefield>, Without<cards::Indestructible>)>,
    zones: zones::ZoneLookup,
) {
    for destruction in destructions.read() {
        let permanent = **destruction;
        let Ok(owner) = cards.get(permanent) else {
            continue;
        };
        if let Some(graveyard) = zones.find(GRAVEYARD, Some(**owner)) {
            commands.entity(permanent).insert(zones::InZone(graveyard));
            **state_based_actions_performed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::damage;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn survives_murder() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MURDER]).mana_pool(mana![1, B, B]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let [myr] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR)?;
        game.play([
            p1.cast(murder).targeting(myr).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        assert_eq!(game.zone_of(murder), GRAVEYARD);
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(DARKSTEEL_MYR));
        Ok(())
    }

    #[test]
    fn survives_lethal_damage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [myr] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR)?;
        game.play([
            p1.cast(bolt).targeting(myr).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        let damage = game.app.world().get::<damage::Damage>(myr.into()).unwrap();
        assert_eq!(**damage, 3);
        Ok(())
    }

    #[test]
    fn survives_doom_blade() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [myr] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR)?;
        game.play([
            p1.cast(blade).targeting(myr).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        assert_eq!(game.zone_of(blade), GRAVEYARD);
        Ok(())
    }
}
