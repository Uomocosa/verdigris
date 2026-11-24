use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::rules;
use crate::zones;
use crate::{GRAVEYARD, LIBRARY};

/// 701.17a For a player to mill a number of cards, that player puts that many cards from the top
/// of their library into their graveyard.
///
/// 701.17b A player can’t mill a number of cards greater than the number of cards in their library.
/// If given the choice to do so, they can’t choose to take that action. If instructed to do so,
/// they mill as many as possible. Similarly, the player can’t pay a cost that includes milling a
/// number of cards greater than the number of cards in their library.
pub fn r701_17a_r701_17b_mill(
    mut mills: MessageReader<rules::Proceeds<rules::Mill>>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for mill in mills.read() {
        let Some(library) = zones.find(LIBRARY, Some(*mill.player)) else {
            continue;
        };
        let Some(graveyard) = zones.find(GRAVEYARD, Some(*mill.player)) else {
            continue;
        };
        let milled_from_top: Vec<Entity> = zone_contents
            .get(library)
            .map(|contents| {
                let requested = usize::try_from(mill.amount).unwrap_or(usize::MAX);
                contents.iter().take(requested).copied().collect()
            })
            .unwrap_or_default();
        for card in milled_from_top {
            commands.entity(card).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND, LIBRARY};

    #[test]
    fn tome_scour_mills_the_top_five_cards() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TOME_SCOUR]).mana_pool(mana![U]),
            P2.library([
                GRIZZLY_BEARS,
                WALKING_CORPSE,
                WATCHWOLF,
                TYPHOID_RATS,
                WIND_DRAKE,
                FOREST,
            ]),
        ]);
        let [tome_scour] = game.player(p1).cards_in(HAND).exactly::<1>(TOME_SCOUR)?;
        let [bears] = game.player(p2).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let [corpse] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WALKING_CORPSE)?;
        let [wolf] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WATCHWOLF)?;
        let [rats] = game.player(p2).cards_in(LIBRARY).exactly::<1>(TYPHOID_RATS)?;
        let [drake] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WIND_DRAKE)?;
        game.play([
            p1.cast(tome_scour).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p2.arrange_graveyard([drake, rats, wolf, corpse, bears])])?;

        assert_eq!(game.player(p2).cards_in(GRAVEYARD).count(), 5);
        assert_eq!(game.player(p2).cards_in(LIBRARY).count(), 1);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(WALKING_CORPSE));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(WATCHWOLF));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(TYPHOID_RATS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(WIND_DRAKE));
        assert!(game.player(p2).cards_in(LIBRARY).has(FOREST));
        Ok(())
    }

    #[test]
    fn tome_scour_mills_as_many_cards_as_possible_when_the_library_is_short() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TOME_SCOUR]).mana_pool(mana![U]),
            P2.library([GRIZZLY_BEARS, FOREST]),
        ]);
        let [tome_scour] = game.player(p1).cards_in(HAND).exactly::<1>(TOME_SCOUR)?;
        let [bears] = game.player(p2).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p2).cards_in(LIBRARY).exactly::<1>(FOREST)?;
        game.play([
            p1.cast(tome_scour).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p2.arrange_graveyard([forest, bears])])?;

        assert_eq!(game.player(p2).cards_in(GRAVEYARD).count(), 2);
        assert_eq!(game.player(p2).cards_in(LIBRARY).count(), 0);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(FOREST));
        Ok(())
    }
}
