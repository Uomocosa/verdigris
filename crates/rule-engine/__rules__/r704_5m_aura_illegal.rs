use bevy::prelude::{Entity, MessageWriter, Query, With};

use crate::Attempt;
use crate::BATTLEFIELD;
use crate::cards;
use crate::rules;
use crate::zones;

/// 704.5m If an Aura is attached to an illegal object or player, or is not attached to an object or
/// player, that Aura is put into its owner’s graveyard.
pub fn r704_5m_aura_illegal(
    mut burials: MessageWriter<Attempt<rules::PutIntoGraveyard>>,
    auras: Query<(Entity, &cards::Owner, &cards::Enchanting), With<cards::OnBattlefield>>,
    enchanted_zones: Query<&zones::InZone>,
    zones: zones::ZoneLookup,
) {
    for (aura, owner, enchanted) in auras.iter() {
        let Ok(enchanted_zone) = enchanted_zones.get(**enchanted) else {
            continue;
        };
        if zones.contains(BATTLEFIELD, **enchanted_zone) {
            continue;
        }
        burials.write(Attempt(rules::PutIntoGraveyard {
            object: aura,
            owner: **owner,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn aura_goes_to_the_graveyard_when_its_creature_dies() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([HOLY_STRENGTH]).mana_pool(mana![W]),
            P2.hand([MURDER]).mana_pool(mana![1, B, B]).battlefield([GRIZZLY_BEARS]),
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

        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(strength));
        Ok(())
    }
}
