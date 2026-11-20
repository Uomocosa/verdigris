use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res};

use crate::actions;
use crate::cards;
use crate::rules::helpers;
use crate::turns;
use crate::zones;

/// 305.6 The basic land types are Plains, Island, Swamp, Mountain, and Forest. If an object uses
/// the words "basic land type," it's referring to one of these subtypes. An object with the land
/// card type and a basic land type has the intrinsic ability "{T}: Add [mana symbol]," even if
/// the text box doesn't actually contain that text or the object has no text box. For Plains,
/// [mana symbol] is {W}; for Islands, {U}; for Swamps, {B}; for Mountains, {R}; and for Forests,
/// {G}. See rule 107.4a. See also rule 605, "Mana Abilities."
///
/// 605.1a An activated ability is a mana ability if it meets all of the following criteria: it
/// doesn't require a target (see rule 115.6), it could add mana to a player's mana pool when it
/// resolves, and it's not a loyalty ability. (See rule 606, "Loyalty Abilities.")
///
/// 605.3a A player may activate an activated mana ability whenever they have priority, whenever
/// they are casting a spell or activating an ability that requires a mana payment, or whenever a
/// rule or effect asks for a mana payment, even if it's in the middle of casting or resolving a
/// spell or activating or resolving an ability.
///
/// 605.3b An activated mana ability doesn't go on the stack, so it can't be targeted, countered,
/// or otherwise responded to. Rather, it resolves immediately after it is activated. (See rule
/// 405.6c.)
///
/// 107.5 The tap symbol is {T}. The tap symbol in an activation cost means "Tap this permanent."
/// A permanent that's already tapped can't be tapped again to pay the cost.
///
/// 118.3 A player can't pay a cost without having the necessary resources to pay it fully. For
/// example, a player with only 1 life can't pay a cost of 2 life, and a permanent that's already
/// tapped can't be tapped to pay a cost.
pub fn r305_6_basic_land_mana_ability(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    permanents: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&cards::Tapped>,
    )>,
    mut pools: Query<&mut cards::ManaPool>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    let priority = priority.into_inner();
    for action in actions.read() {
        let actions::Action::ActivateManaAbility { source, produces } = &action.action else {
            continue;
        };
        let produces = *produces;
        let permanent = *source;
        let Ok((definition, owner, in_zone, tapped)) = permanents.get(permanent) else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        let is_on_the_battlefield =
            helpers::is_card_in_zone(**in_zone, crate::BATTLEFIELD, &zone_entities);
        let Some(land_type) =
            definition.land_types.iter().copied().find(|land_type| land_type.color() == produces)
        else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        if action.player != priority.player
            || **owner != action.player
            || !is_on_the_battlefield
            || tapped.is_some()
        {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        }
        let Ok(mut pool) = pools.get_mut(action.player) else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        *pool = pool.plus(land_type.mana_symbol());
        commands.entity(permanent).insert(cards::Tapped);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn test_usage() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        game.play([p1.activate_ability(forest.ability())])?;
        assert_eq!(game.mana_pool(p1), mana![G]);
        Ok(())
    }

    #[test]
    fn r305_6_a_forest_taps_for_green_mana_without_using_the_stack() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);

        game.play([p1.activate_ability(forest.ability())])?;

        assert_eq!(game.mana_pool(p1), mana![G]);
        assert!(game.last_rejection().is_none());
        assert!(game.app.world().get::<cards::Tapped>(forest.into()).is_some());
        assert_eq!(game.zone_of(forest), BATTLEFIELD);
        Ok(())
    }

    #[test]
    fn r305_6_a_mountain_taps_for_red_mana() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([MOUNTAIN]), P2]);
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;

        game.play([p1.activate_ability(mountain.ability())])?;

        assert_eq!(game.mana_pool(p1), mana![R]);
        Ok(())
    }

    #[test]
    fn r107_5_a_tapped_land_cannot_be_tapped_for_more_mana() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::Color::Green,
            },
        );

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::Color::Green,
            },
        );

        assert_eq!(game.mana_pool(p1), mana![G]);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: forest.into()
            })
        );
        Ok(())
    }

    #[test]
    fn r305_6_a_forest_asked_for_white_is_rejected_and_stays_untapped() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::Color::White,
            },
        );

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert!(game.app.world().get::<cards::Tapped>(forest.into()).is_none());
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: forest.into()
            })
        );
        Ok(())
    }

    #[test]
    fn r305_6_a_tundra_taps_for_white_mana() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        game.play([p1.activate_ability(tundra.mana_ability(cards::Color::White))])?;

        assert_eq!(game.mana_pool(p1), mana![W]);
        assert!(game.last_rejection().is_none());
        assert!(game.app.world().get::<cards::Tapped>(tundra.into()).is_some());
        Ok(())
    }

    #[test]
    fn r305_6_a_second_tundra_taps_for_blue_mana() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA, TUNDRA]), P2]);
        let [first, second] = game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(TUNDRA)?;

        game.play([p1
            .activate_ability(first.mana_ability(cards::Color::White))
            .activate_ability(second.mana_ability(cards::Color::Blue))])?;

        assert_eq!(game.mana_pool(p1), mana![W, U]);
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn r305_6_a_tapped_tundra_cannot_be_tapped_for_the_other_colour() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tundra.into(),
                produces: cards::Color::White,
            },
        );
        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tundra.into(),
                produces: cards::Color::Blue,
            },
        );

        assert_eq!(game.mana_pool(p1), mana![W]);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: tundra.into()
            })
        );
        Ok(())
    }
}
