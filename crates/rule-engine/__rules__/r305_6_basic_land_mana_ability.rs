use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res, With};

use crate::actions;
use crate::cards;
use crate::turns;

/// 305.6. The basic land types are Plains, Island, Swamp, Mountain, and Forest. If an object uses
/// the words “basic land type,” it’s referring to one of these subtypes. An object with the land
/// card type and a basic land type has the intrinsic ability “{T}: Add [mana symbol],” even if the
/// text box doesn’t actually contain that text or the object has no text box. For Plains, [mana
/// symbol] is {W}; for Islands, {U}; for Swamps, {B}; for Mountains, {R}; and for Forests, {G}. See
/// rule 107.4a. See also rule 605, “Mana Abilities.”
///
/// # See also
///
/// 605.1a An activated ability is a mana ability if it meets all of the following criteria: it
/// doesn’t require a target (see rule 115.6), it could add mana to a player’s mana pool when it
/// resolves, and it’s not a loyalty ability. (See rule 606, “Loyalty Abilities.”)
///
/// 605.3a A player may activate an activated mana ability whenever they have priority, whenever
/// they are casting a spell or activating an ability that requires a mana payment, or whenever a
/// rule or effect asks for a mana payment, even if it’s in the middle of casting or resolving a
/// spell or activating or resolving an ability.
///
/// 605.3b An activated mana ability doesn’t go on the stack, so it can’t be targeted, countered, or
/// otherwise responded to. Rather, it resolves immediately after it is activated. (See rule
/// 405.6c.)
///
/// 107.5. The tap symbol is {T}. The tap symbol in an activation cost means “Tap this permanent.” A
/// permanent that’s already tapped can’t be tapped again to pay the cost. A creature’s activated
/// ability with the tap symbol in its activation cost can’t be activated unless the creature has
/// been under its controller’s control continuously since their most recent turn began. See rule
/// 302.6.
///
/// 118.3. A player can’t pay a cost without having the necessary resources to pay it fully. For
/// example, a player with only 1 life can’t pay a cost of 2 life, and a permanent that’s already
/// tapped can’t be tapped to pay a cost. See rule 202, “Mana Cost and Color,” and rule 602,
/// “Activating Activated Abilities.”
pub fn r305_6_basic_land_mana_ability(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    window: Res<actions::OpeningHandWindow>,
    turn_order: Res<turns::TurnOrder>,
    permanents: Query<
        (
            &cards::CardDefinition,
            &cards::Owner,
            Option<&cards::Tapped>,
        ),
        With<cards::OnBattlefield>,
    >,
    mut pools: Query<&mut cards::ManaPool>,
) {
    let priority = priority.into_inner();
    let window = window.into_inner();
    let turn_order = turn_order.into_inner();
    for action in actions.read() {
        let actions::Action::ActivateManaAbility { source, produces } = &action.action else {
            continue;
        };
        let produces = *produces;
        let permanent = *source;
        if !window.is_complete(turn_order.len()) {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        }
        let Ok((definition, owner, tapped)) = permanents.get(permanent) else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        if definition.activated_abilities().any(|ability| ability.produces == produces) {
            continue;
        }
        let cards::ManaType::Color(color) = produces else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        let Some(land_type) =
            definition.type_line.land_types().find(|land_type| land_type.color() == color)
        else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        if action.player != priority.player || **owner != action.player || tapped.is_some() {
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
    fn a_forest_taps_for_green_mana_without_using_the_stack() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);

        game.play([p1.activate_ability(forest.ability())])?;

        assert_eq!(game.mana_pool(p1), mana![G]);
        assert!(game.last_rejection().is_none());
        assert!(game.is_tapped(forest));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(forest));
        Ok(())
    }

    #[test]
    fn a_mountain_taps_for_red_mana() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([MOUNTAIN]), P2]);
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;

        game.play([p1.activate_ability(mountain.ability())])?;

        assert_eq!(game.mana_pool(p1), mana![R]);
        Ok(())
    }

    #[test]
    fn a_tapped_land_cannot_be_tapped_for_more_mana() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::ManaType::from(cards::Color::Green),
            },
        );

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::ManaType::from(cards::Color::Green),
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
    fn a_forest_asked_for_white_is_rejected_and_stays_untapped() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::ManaType::from(cards::Color::White),
            },
        );

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert!(!game.is_tapped(forest));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: forest.into()
            })
        );
        Ok(())
    }

    #[test]
    fn a_tundra_taps_for_white_mana() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        game.play([
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::White)))
        ])?;

        assert_eq!(game.mana_pool(p1), mana![W]);
        assert!(game.last_rejection().is_none());
        assert!(game.is_tapped(tundra));
        Ok(())
    }

    #[test]
    fn a_second_tundra_taps_for_blue_mana() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA, TUNDRA]), P2]);
        let [tundra_1, tundra_2] = game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(TUNDRA)?;

        game.play([p1
            .activate_ability(tundra_1.mana_ability(cards::ManaType::from(cards::Color::White)))
            .activate_ability(tundra_2.mana_ability(cards::ManaType::from(cards::Color::Blue)))])?;

        assert_eq!(game.mana_pool(p1), mana![W, U]);
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn a_tapped_tundra_cannot_be_tapped_for_the_other_colour() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tundra.into(),
                produces: cards::ManaType::from(cards::Color::White),
            },
        );
        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tundra.into(),
                produces: cards::ManaType::from(cards::Color::Blue),
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
