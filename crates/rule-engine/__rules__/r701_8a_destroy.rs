use crate::events;
use bevy::prelude::{MessageReader, MessageWriter, Query, With};

use crate::Attempt;
use crate::Resolve;
use crate::cards;

/// 701.8a To destroy a permanent, move it from the battlefield to its owner’s graveyard.
///
/// # See also
///
/// 702.12b A permanent with indestructible can’t be destroyed. Such permanents aren’t destroyed by
/// lethal damage, and they ignore the state-based action that checks for lethal damage (see rule
/// 704.5g).
pub fn r701_8a_destroy(
    mut destructions: MessageReader<Resolve<events::Destroy>>,
    mut moves_to_graveyard: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    permanents: Query<&cards::Owner, With<cards::OnBattlefield>>,
) {
    for destruction in destructions.read() {
        let Ok(owner) = permanents.get(destruction.permanent) else {
            continue;
        };
        moves_to_graveyard.write(Attempt(events::PutIntoGraveyard {
            object: destruction.permanent,
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
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(myr));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(murder));
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
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(myr));
        assert_eq!(game.damage(myr), 3);
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
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(myr));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(blade));
        Ok(())
    }

    #[test]
    fn safe_haven_stops_murder_from_destroying_the_protected_creature() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MURDER]).mana_pool(mana![1, B, B]),
            P2.battlefield([TEST_SAFE_HAVEN, GRIZZLY_BEARS]),
        ]);
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(murder).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(murder));
        Ok(())
    }
}
