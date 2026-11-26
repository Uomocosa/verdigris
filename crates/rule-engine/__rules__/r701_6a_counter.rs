use crate::STACK;
use crate::events;
use bevy::prelude::{MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Resolve;
use crate::cards;
use crate::zones;

/// 701.6a To counter a spell or ability means to cancel it, removing it from the stack. It doesn’t
/// resolve and none of its effects occur. A countered spell is put into its owner’s graveyard.
pub fn r701_6a_counter(
    mut countered: MessageReader<Resolve<events::Counter>>,
    mut moves_to_graveyard: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    spells: Query<(&cards::Owner, &zones::InZone)>,
    zones: zones::ZoneLookup,
) {
    for event in countered.read() {
        let spell = event.spell;
        let Ok((owner, in_zone)) = spells.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        moves_to_graveyard.write(Attempt(events::PutIntoGraveyard {
            object: spell,
            owner: **owner,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn cancel_counters_bolt_without_damage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [cancel] = game.player(p2).cards_in(HAND).exactly::<1>(CANCEL)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 20);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(cancel));
        Ok(())
    }

    #[test]
    fn essence_scatter_counters_a_creature_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2.hand([ESSENCE_SCATTER]).mana_pool(mana![1, U]),
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [scatter] = game.player(p2).cards_in(HAND).exactly::<1>(ESSENCE_SCATTER)?;

        game.play([
            p1.cast(bears).pass_priority(),
            p2.cast(scatter).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(scatter));
        Ok(())
    }

    #[test]
    fn a_countered_spell_that_is_never_buried_does_not_enter_its_owners_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TEST_NEVER_BURIED]).mana_pool(mana![1, G]),
            P2.hand([ESSENCE_SCATTER]).mana_pool(mana![1, U]),
        ]);
        let [spell] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_NEVER_BURIED)?;
        let [scatter] = game.player(p2).cards_in(HAND).exactly::<1>(ESSENCE_SCATTER)?;

        game.play([
            p1.cast(spell).pass_priority(),
            p2.cast(scatter).targeting(spell).pass_priority(),
            p1.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has_no(spell));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(scatter));
        Ok(())
    }

    #[test]
    fn negate_counters_a_noncreature_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [negate] = game.player(p2).cards_in(HAND).exactly::<1>(NEGATE)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(negate).targeting(bolt).pass_priority(),
            p1.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 20);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(negate));
        Ok(())
    }

    #[test]
    fn dispel_can_counter_a_cancel_so_the_outer_bolt_resolves() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DISPEL]).mana_pool(mana![R, U]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [dispel] = game.player(p1).cards_in(HAND).exactly::<1>(DISPEL)?;
        let [cancel] = game.player(p2).cards_in(HAND).exactly::<1>(CANCEL)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.cast(dispel).targeting(cancel).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(cancel));
        assert_eq!(game.life(p2), 20);

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p2), 17);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(dispel));
        Ok(())
    }

    #[test]
    fn cancelled_cancel_does_not_stop_the_outer_bolt() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, CANCEL]).mana_pool(mana![1, U, U, R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [p1_cancel] = game.player(p1).cards_in(HAND).exactly::<1>(CANCEL)?;
        let [p2_cancel] = game.player(p2).cards_in(HAND).exactly::<1>(CANCEL)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(p2_cancel).targeting(bolt).pass_priority(),
            p1.cast(p1_cancel).targeting(p2_cancel).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(p2_cancel));
        assert_eq!(game.life(p2), 20);

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.life(p2), 17);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(p1_cancel));
        Ok(())
    }
}
