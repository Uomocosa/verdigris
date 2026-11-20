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
        let actions::Action::ActivateManaAbility(permanent) = &action.action else {
            continue;
        };
        let permanent = *permanent;
        let Ok((definition, owner, in_zone, tapped)) = permanents.get(permanent) else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        let is_on_the_battlefield =
            helpers::is_card_in_zone(**in_zone, crate::BATTLEFIELD, &zone_entities);
        let [land_type] = definition.land_types.as_slice() else {
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
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(FOREST)
            .exactly::<1>()
            .unwrap();
        game.act(p1, actions::Action::ActivateManaAbility(forest.into()));
        assert_eq!(game.mana_pool(p1), mana![G]);
    }

    #[test]
    fn r305_6_a_forest_taps_for_green_mana_without_using_the_stack() {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(FOREST)
            .exactly::<1>()
            .unwrap();
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);

        game.act(p1, actions::Action::ActivateManaAbility(forest.into()));

        assert_eq!(game.mana_pool(p1), mana![G]);
        assert!(game.last_rejection().is_none());
        assert!(
            game.app
                .world()
                .get::<cards::Tapped>(forest.into())
                .is_some()
        );
        assert_eq!(game.zone_of(forest), BATTLEFIELD);
    }

    #[test]
    fn r305_6_a_mountain_taps_for_red_mana() {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([MOUNTAIN]), P2]);
        let [mountain] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(MOUNTAIN)
            .exactly::<1>()
            .unwrap();

        game.act(p1, actions::Action::ActivateManaAbility(mountain.into()));

        assert_eq!(game.mana_pool(p1), mana![R]);
    }

    #[test]
    fn r107_5_a_tapped_land_cannot_be_tapped_for_more_mana() {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(FOREST)
            .exactly::<1>()
            .unwrap();
        game.act(p1, actions::Action::ActivateManaAbility(forest.into()));

        game.act(p1, actions::Action::ActivateManaAbility(forest.into()));

        assert_eq!(game.mana_pool(p1), mana![G]);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: forest.into()
            })
        );
    }
}
