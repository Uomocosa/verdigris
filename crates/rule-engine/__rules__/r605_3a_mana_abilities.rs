use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res, With};

use crate::abilities;
use crate::actions;
use crate::cards;
use crate::turns;
/// 605.3a A player may activate an activated mana ability whenever they have priority, whenever
/// they are casting a spell or activating an ability that requires a mana payment, or whenever a
/// rule or effect asks for a mana payment, even if it’s in the middle of casting or resolving a
/// spell or activating or resolving an ability.
pub fn r605_3a_mana_abilities(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    state: Res<turns::TurnState>,
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
    let state = state.into_inner();
    for action in actions.read() {
        let actions::Action::ActivateManaAbility { source, produces } = &action.action else {
            continue;
        };
        let produces = *produces;
        let permanent = *source;
        let Ok((definition, owner, tapped)) = permanents.get(permanent) else {
            continue;
        };
        let Some(ability) =
            definition.activated_abilities().find(|ability| ability.produces == produces)
        else {
            continue;
        };
        if ability.cost != abilities::ActivationCost::Tap {
            continue;
        }
        if state.priority.holder != Some(action.player)
            || **owner != action.player
            || tapped.is_some()
        {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        }
        let Ok(mut pool) = pools.get_mut(*action.player) else {
            rejected.write(actions::Rejected::CannotActivateManaAbility { permanent });
            continue;
        };
        *pool = pool.plus(produces.mana_symbol());
        commands.entity(permanent).insert(cards::Tapped);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn reliquary_tower_taps_for_one_colorless_mana() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([RELIQUARY_TOWER]), P2]);
        let [tower] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(RELIQUARY_TOWER)?;

        game.play([p1.activate_ability(tower.mana_ability(cards::ManaType::Colorless))])?;

        assert_eq!(game.mana_pool(p1), mana![C]);
        assert!(game.last_rejection().is_none());
        assert!(game.is_tapped(tower));
        Ok(())
    }

    #[test]
    fn a_tapped_reliquary_tower_cannot_tap_for_colorless() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([RELIQUARY_TOWER]), P2]);
        let [tower] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(RELIQUARY_TOWER)?;
        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tower.into(),
                produces: cards::ManaType::Colorless,
            },
        );

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tower.into(),
                produces: cards::ManaType::Colorless,
            },
        );

        assert_eq!(game.mana_pool(p1), mana![C]);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: tower.into()
            })
        );
        Ok(())
    }

    #[test]
    fn reliquary_tower_cannot_tap_without_priority() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([RELIQUARY_TOWER]), P2]);
        let [tower] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(RELIQUARY_TOWER)?;
        game.play([p1.pass_priority()])?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tower.into(),
                produces: cards::ManaType::Colorless,
            },
        );

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert!(!game.is_tapped(tower));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: tower.into()
            })
        );
        Ok(())
    }

    #[test]
    fn reliquary_tower_asked_for_green_is_rejected_once_and_stays_untapped() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([RELIQUARY_TOWER]), P2]);
        let [tower] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(RELIQUARY_TOWER)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: tower.into(),
                produces: cards::ManaType::from(cards::Color::Green),
            },
        );

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert!(!game.is_tapped(tower));
        assert_eq!(game.rejection_count(), 1);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: tower.into()
            })
        );
        Ok(())
    }

    #[test]
    fn three_towers_pay_the_generic_cost_of_darksteel_myr() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DARKSTEEL_MYR]).battlefield([
                RELIQUARY_TOWER,
                RELIQUARY_TOWER,
                RELIQUARY_TOWER,
            ]),
            P2,
        ]);
        let [tower_1, tower_2, tower_3] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<3>(RELIQUARY_TOWER)?;
        let [myr] = game.player(p1).cards_in(HAND).exactly::<1>(DARKSTEEL_MYR)?;

        game.play([
            p1.activate_ability(tower_1.mana_ability(cards::ManaType::Colorless))
                .activate_ability(tower_2.mana_ability(cards::ManaType::Colorless))
                .activate_ability(tower_3.mana_ability(cards::ManaType::Colorless)),
            p1.cast(myr).end_turn(),
            p2.end_turn(),
        ])?;

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(myr));
        Ok(())
    }
}
