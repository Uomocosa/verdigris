use crate::events;
use bevy::prelude::{Entity, MessageWriter, Query, With};

use crate::Attempt;
use crate::cards;
use crate::rules;

const CAUSE: rules::EventId = rules::EventId("704.5f/zero-toughness");

/// 704.5f If a creature has toughness 0 or less, it’s put into its owner’s graveyard. Regeneration
/// can’t replace this event.
pub fn r704_5f_zero_toughness(
    mut burials: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    cards: Query<(Entity, &cards::EffectivePT), With<cards::OnBattlefield>>,
) {
    for (creature, effective) in cards.iter() {
        if effective.toughness <= 0 {
            burials.write(Attempt(events::PutIntoGraveyard {
                object: creature,
                cause: CAUSE,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn disfigured_creature_with_zero_toughness_goes_to_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE]).mana_pool(mana![B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [disfigure] = game.player(p1).cards_in(HAND).exactly::<1>(DISFIGURE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(!game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert!(game.moved(disfigure).to(GRAVEYARD).of(p1));
        Ok(())
    }

    #[test]
    fn a_cant_be_put_into_a_graveyard_effect_keeps_the_zero_toughness_creature() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE, MURDER]).mana_pool(mana![1, B, B, B]),
            P2.battlefield([GRIZZLY_BEARS, TEST_NO_BURIALS]),
        ]);
        let [disfigure] = game.player(p1).cards_in(HAND).exactly::<1>(DISFIGURE)?;
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
            p1.cast(murder).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert!(game.moved(disfigure).to(GRAVEYARD).of(p1));
        assert!(game.moved(murder).to(GRAVEYARD).of(p1));
        Ok(())
    }
}
