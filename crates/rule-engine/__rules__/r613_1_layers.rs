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
/// 613.1f Layer 6: Ability-adding effects, keyword counters, ability-removing effects, and
/// effects that say an object can’t have an ability are applied.
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
    zones: zones::ZoneLookup,
) {
    for (card, definition, in_zone, current) in cards.iter() {
        let on_battlefield = zones.contains(BATTLEFIELD, **in_zone);
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
    use crate::effects;
    use crate::keywords;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn battlefield_cards_get_derived_markers() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]).hand([GIANT_GROWTH]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        game.pass_priority();
        assert!(game.has::<cards::OnBattlefield>(bears));
        assert_eq!(game.pt(bears), Some(crate::pt!(2, 2)));
        assert!(!game.has::<cards::OnBattlefield>(growth));
        assert!(game.pt(growth).is_none());
        Ok(())
    }

    #[test]
    fn jump_grants_flying_until_end_of_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([JUMP]).battlefield([GRIZZLY_BEARS]).mana_pool(mana![U]),
            P2,
        ]);
        let [jump] = game.player(p1).cards_in(HAND).exactly::<1>(JUMP)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(jump).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.has::<cards::Flying>(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(jump));
        Ok(())
    }

    #[test]
    fn flying_stays_while_a_second_jump_is_active() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([JUMP, JUMP]).battlefield([GRIZZLY_BEARS]).mana_pool(mana![U, U]),
            P2,
        ]);
        let [first, second] = game.player(p1).cards_in(HAND).exactly::<2>(JUMP)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(first).targeting(bears),
            p1.cast(second).targeting(bears).pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;
        let grants = game.with_query::<&effects::ContinuousEffect, usize>(|granted| {
            granted
                .iter()
                .filter(|effect| {
                    effect.affected == bears
                        && effect.change == effects::Layered::GainKeyword(keywords::Keyword::Flying)
                })
                .count()
        });
        assert_eq!(grants, 2);
        assert!(game.has::<cards::Flying>(bears));
        Ok(())
    }

    #[test]
    fn flying_ends_at_cleanup() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([JUMP]).battlefield([GRIZZLY_BEARS]).mana_pool(mana![U]),
            P2,
        ]);
        let [jump] = game.player(p1).cards_in(HAND).exactly::<1>(JUMP)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(jump).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.has::<cards::Flying>(bears));
        game.play([p1.end_turn(), p2.end_turn()])?;
        assert!(!game.has::<cards::Flying>(bears));
        Ok(())
    }

    #[test]
    fn later_timestamp_applies_last() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GIANT_GROWTH, JUMP]).battlefield([GRIZZLY_BEARS]).mana_pool(mana![G, U]),
            P2,
        ]);
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        let [jump] = game.player(p1).cards_in(HAND).exactly::<1>(JUMP)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(growth).targeting(bears),
            p1.cast(jump).targeting(bears).pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;
        let timestamps = game.with_query::<&effects::ContinuousEffect, Vec<u64>>(|granted| {
            granted.iter().map(|effect| effect.timestamp).collect()
        });
        assert_eq!(timestamps.len(), 2);
        assert!(timestamps[0] < timestamps[1]);
        assert_eq!(game.pt(bears), Some(crate::pt!(5, 5)));
        assert!(game.has::<cards::Flying>(bears));
        Ok(())
    }

    #[test]
    #[ignore = "needs a dependency card: 613.8"]
    fn dependency_overrides_timestamp() {}
}
