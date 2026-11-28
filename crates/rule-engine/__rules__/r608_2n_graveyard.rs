use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, With};

use crate::Attempt;
use crate::RESOLUTION_CAUSE;
use crate::STACK;
use crate::cards;
use crate::stack;
use crate::zones;

/// 608.2n As the final part of an instant or sorcery spell’s resolution, the spell is put into its
/// owner’s graveyard. As the final part of an ability’s resolution, the ability is removed from the
/// stack and ceases to exist.
pub fn r608_2n_graveyard(
    mut resolutions: MessageReader<stack::Resolution>,
    mut graveyards: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    mut commands: Commands,
    abilities: Query<&zones::InZone, With<stack::TriggeredAbilityOnStack>>,
    spells: Query<(&cards::CardDefinition, &zones::InZone)>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        if let Ok(in_zone) = abilities.get(spell) {
            if zones.contains(STACK, **in_zone) {
                commands.entity(spell).despawn();
            }
            continue;
        }
        let Ok((definition, in_zone)) = spells.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        let is_instant_or_sorcery = definition.type_line.types.iter().any(|card_type| {
            matches!(
                card_type,
                cards::CardType::Instant | cards::CardType::Sorcery
            )
        });
        if !is_instant_or_sorcery {
            continue;
        }
        graveyards.write(Attempt(events::PutIntoGraveyard {
            object: spell,
            cause: RESOLUTION_CAUSE,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, EXILE, GRAVEYARD, HAND};

    #[test]
    fn resolved_instant_goes_to_graveyard() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(bolt).to(GRAVEYARD).of(p1));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(LIGHTNING_BOLT));
        Ok(())
    }

    #[test]
    fn leyline_of_the_void_exiles_a_resolved_instant_instead_of_burying_it() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([LEYLINE_OF_THE_VOID]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(bolt).to(EXILE).of(p1));
        assert!(!game.card(bolt).is_in(GRAVEYARD).of(p1));
        Ok(())
    }

    #[test]
    fn leyline_of_the_void_exiles_an_opponents_discarded_card() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]).battlefield([LEYLINE_OF_THE_VOID]),
            P2.hand([GRIZZLY_BEARS]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;
        assert!(game.moved(bears).to(EXILE).of(p2));
        assert!(!game.card(bears).is_in(GRAVEYARD).of(p2));
        Ok(())
    }

    #[test]
    fn leyline_of_the_void_exiles_a_card_an_opponent_owns_but_you_control() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MURDER]).mana_pool(mana![1, B, B]),
            P2.battlefield([LEYLINE_OF_THE_VOID]).battlefield_owned_by(&P1, [GRIZZLY_BEARS]),
        ]);
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(murder).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(bears).to(EXILE).of(p1));
        assert!(!game.card(bears).is_in(GRAVEYARD).of(p1));
        Ok(())
    }

    #[test]
    fn leyline_of_the_void_buries_a_card_you_own_that_an_opponent_controls() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([LEYLINE_OF_THE_VOID]),
            P2.hand([MURDER])
                .mana_pool(mana![1, B, B])
                .battlefield_owned_by(&P1, [GRIZZLY_BEARS])
                .active(),
        ]);
        let [murder] = game.player(p2).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p2.cast(murder).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        assert!(!game.card(bears).is_in(EXILE).of(p1));
        Ok(())
    }
}
