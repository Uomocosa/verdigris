use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, MessageReader, Query};

use crate::cards;
use crate::stack;
use crate::zones;

/// 608.2n As the final part of an instant or sorcery spell’s resolution, the spell is put into its
/// owner’s graveyard. As the final part of an ability’s resolution, the ability is removed from the
/// stack and ceases to exist.
pub fn r608_2n_graveyard(
    mut resolutions: MessageReader<stack::Resolution>,
    mut commands: Commands,
    cards: Query<(
        Option<&cards::CardDefinition>,
        Option<&stack::TriggeredAbilityOnStack>,
        &cards::Owner,
        &zones::InZone,
    )>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, ability, owner, in_zone)) = cards.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        if ability.is_some() {
            commands.entity(spell).despawn();
            continue;
        }
        let Some(definition) = definition else {
            continue;
        };
        let is_instant_or_sorcery = definition.types.iter().any(|card_type| {
            matches!(
                card_type,
                cards::CardType::Instant | cards::CardType::Sorcery
            )
        });
        if !is_instant_or_sorcery {
            continue;
        }
        if let Some(graveyard) = zones.find(GRAVEYARD, Some(**owner)) {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn resolved_instant_goes_to_graveyard() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(LIGHTNING_BOLT));
        Ok(())
    }
}
