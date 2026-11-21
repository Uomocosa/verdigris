use bevy::prelude::{Commands, Entity, Query};

use crate::BATTLEFIELD;
use crate::cards;
use crate::effects;
use crate::zones;

/// 613.1. The values of an object’s characteristics are determined by starting with the actual
/// object. For a card, that means the values of the characteristics printed on that card. For a
/// token or a copy of a spell or card, that means the values of the characteristics defined by the
/// effect that created it. Then all applicable continuous effects are applied in a series of layers
/// in the following order:
///
/// # See also
///
/// 613.4c Layer 7c: Effects and counters that modify power and/or toughness (but don’t set power
/// and/or toughness to a specific number or value) are applied.
///
/// 613.5. The application of continuous effects as described by the layer system is continually and
/// automatically performed by the game. All resulting changes to an object’s characteristics are
/// instantaneous.
///
/// 613.7. Within a layer or sublayer, determining which order effects are applied in is usually
/// done using a timestamp system. An effect with an earlier timestamp is applied before an effect
/// with a later timestamp.
pub fn r613_1_layers(
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &zones::InZone,
        cards::CurrentMarkers<'_>,
    )>,
    locked: Query<&effects::ContinuousEffect>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for (card, definition, in_zone, current) in cards.iter() {
        let on_battlefield = BATTLEFIELD.contains_card(**in_zone, &zone_entities);
        let mut applicable: Vec<effects::ContinuousEffect> =
            locked.iter().filter(|effect| effect.affected == card).copied().collect();
        applicable.sort_by_key(|effect| effect.timestamp);
        let characteristics = cards::Characteristics::from_printed(definition, on_battlefield)
            .with_effects(&applicable);
        characteristics.write_markers(&mut commands.entity(card), current);
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn battlefield_cards_get_derived_markers() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]).hand([GIANT_GROWTH]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        game.pass_priority();
        let world = game.app.world();
        assert!(world.get::<cards::OnBattlefield>(bears.into()).is_some());
        assert_eq!(
            world.get::<cards::EffectivePT>(bears.into()).copied(),
            Some(cards::EffectivePT(crate::pt!(2, 2)))
        );
        assert!(world.get::<cards::OnBattlefield>(growth.into()).is_none());
        assert!(world.get::<cards::EffectivePT>(growth.into()).is_none());
        Ok(())
    }
}
