use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, With};

use crate::Attempt;
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
        graveyards.write(Attempt(events::PutIntoGraveyard { object: spell }));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{EXILE, GRAVEYARD, HAND};

    #[test]
    fn resolved_instant_goes_to_graveyard() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.card(bolt).is_in(GRAVEYARD).of(p1));
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
        assert!(game.card(bolt).is_in(EXILE).of(p1));
        assert!(!game.card(bolt).is_in(GRAVEYARD).of(p1));
        Ok(())
    }
}
