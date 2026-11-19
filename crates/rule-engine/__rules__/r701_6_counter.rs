use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 701.6a To counter a spell or ability means to cancel it, removing it from the stack. It
/// doesn't resolve and none of its effects occur. A countered spell is put into its owner's
/// graveyard.
pub fn r701_6_counter(
    mut countered: MessageReader<stack::Countered>,
    mut commands: Commands,
    spells: Query<(&cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for event in countered.read() {
        let spell = **event;
        let Ok((owner, in_zone)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, STACK, &zone_entities) {
            continue;
        }
        if let Some(graveyard) = helpers::find_zone(GRAVEYARD, Some(**owner), &zone_entities) {
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
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .expect_n::<1>()
            .unwrap();
        let [cancel] = game
            .player(p2)
            .cards_in(HAND)
            .matching(CANCEL)
            .expect_n::<1>()
            .unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(cancel), GRAVEYARD);
    }

    #[test]
    fn essence_scatter_counters_a_creature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2.hand([ESSENCE_SCATTER]).mana_pool(mana![1, U]),
        ]);
        let [bears] = game
            .player(p1)
            .cards_in(HAND)
            .matching(GRIZZLY_BEARS)
            .expect_n::<1>()
            .unwrap();
        let [scatter] = game
            .player(p2)
            .cards_in(HAND)
            .matching(ESSENCE_SCATTER)
            .expect_n::<1>()
            .unwrap();

        game.play([
            p1.cast(bears).pass_priority(),
            p2.cast(scatter).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert_eq!(game.zone_of(scatter), GRAVEYARD);
    }

    #[test]
    fn negate_counters_a_noncreature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let [bolt] = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .expect_n::<1>()
            .unwrap();
        let [negate] = game
            .player(p2)
            .cards_in(HAND)
            .matching(NEGATE)
            .expect_n::<1>()
            .unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(negate).targeting(bolt).pass_priority(),
            p1.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(negate), GRAVEYARD);
    }

    #[test]
    fn dispel_can_counter_a_cancel_so_the_outer_bolt_resolves() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DISPEL]).mana_pool(mana![R, U]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .expect_n::<1>()
            .unwrap();
        let [dispel] = game
            .player(p1)
            .cards_in(HAND)
            .matching(DISPEL)
            .expect_n::<1>()
            .unwrap();
        let [cancel] = game
            .player(p2)
            .cards_in(HAND)
            .matching(CANCEL)
            .expect_n::<1>()
            .unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.cast(dispel).targeting(cancel).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert_eq!(game.zone_of(cancel), GRAVEYARD);
        assert_eq!(game.life(p2), 20);

        game.play([p1.pass_priority(), p2.pass_priority()]).unwrap();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(dispel), GRAVEYARD);
    }

    #[test]
    fn cancelled_cancel_does_not_stop_the_outer_bolt() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, CANCEL])
                .mana_pool(mana![1, U, U, R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .expect_n::<1>()
            .unwrap();
        let [p1_cancel] = game
            .player(p1)
            .cards_in(HAND)
            .matching(CANCEL)
            .expect_n::<1>()
            .unwrap();
        let [p2_cancel] = game
            .player(p2)
            .cards_in(HAND)
            .matching(CANCEL)
            .expect_n::<1>()
            .unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(p2_cancel).targeting(bolt).pass_priority(),
            p1.cast(p1_cancel).targeting(p2_cancel).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert_eq!(game.zone_of(p2_cancel), GRAVEYARD);
        assert_eq!(game.life(p2), 20);

        game.play([p1.pass_priority(), p2.pass_priority()]).unwrap();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(p1_cancel), GRAVEYARD);
    }
}
