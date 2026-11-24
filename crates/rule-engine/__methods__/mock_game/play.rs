use crate::actions;
use crate::cards;
use crate::testing;
use crate::turns;

const MAX_PASSES_PER_PLAY: u32 = 256;

pub fn play(
    game: &mut testing::MockGame,
    actions: impl IntoIterator<Item = testing::TestPlayerActions>,
) -> Result<(), testing::PlayError> {
    let mut auto_passes: Vec<testing::ActiveAutoPass> = Vec::new();
    let mut budget = testing::PassBudget::of(MAX_PASSES_PER_PLAY);
    for (index, step) in actions.into_iter().flat_map(|action| action.steps).enumerate() {
        match &step {
            testing::PlayStep::Act(player, action) => {
                let before = game.rejection_count();
                game.act(*player, action.clone());
                if game.rejection_count() > before
                    && let Some(rejection) = game.last_rejection().cloned()
                {
                    return Err(testing::PlayError::Rejected {
                        index,
                        step: step.clone(),
                        rejection,
                    });
                }
            }
            testing::PlayStep::Activate(player, ability) => {
                let source = match ability {
                    testing::TestAbility::Only { source }
                    | testing::TestAbility::Mana { source, .. } => *source,
                };
                let produces = match ability {
                    testing::TestAbility::Mana { produces, .. } => Some(*produces),
                    testing::TestAbility::Only { .. } => game
                        .app
                        .world()
                        .get::<cards::CardDefinition>(source)
                        .and_then(|definition| {
                            let land_types = definition.type_line.land_types().collect::<Vec<_>>();
                            land_types
                                .first()
                                .filter(|_| land_types.len() == 1)
                                .map(|land| cards::ManaType::from(land.color()))
                        }),
                };
                let Some(produces) = produces else {
                    let count = game
                        .app
                        .world()
                        .get::<cards::CardDefinition>(source)
                        .map_or(0, |definition| definition.type_line.land_types().count());
                    return Err(testing::PlayError::AmbiguousAbility {
                        index,
                        card: source,
                        count,
                    });
                };
                let before = game.rejection_count();
                game.act(
                    *player,
                    actions::Action::ActivateManaAbility { source, produces },
                );
                if game.rejection_count() > before
                    && let Some(rejection) = game.last_rejection().cloned()
                {
                    return Err(testing::PlayError::Rejected {
                        index,
                        step: step.clone(),
                        rejection,
                    });
                }
            }
            testing::PlayStep::AutoPass { player, until } => {
                let start = *game.app.world().resource::<turns::Turn>();
                auto_passes.push(testing::ActiveAutoPass {
                    player: *player,
                    until: *until,
                    start,
                });
            }
        }
        loop {
            let holder = game.priority().holder.unwrap_or_else(|| game.turn().active);
            let Some(position) = auto_passes.iter().position(|pass| pass.player == *holder) else {
                break;
            };
            let now = *game.app.world().resource::<turns::Turn>();
            let Some(auto_pass) = auto_passes.get(position) else {
                break;
            };
            if auto_pass.is_stop_reached(now) {
                auto_passes.remove(position);
                break;
            }
            let before = game.rejection_count();
            game.act(*holder, actions::Action::PassPriority);
            if game.rejection_count() > before
                && let Some(rejection) = game.last_rejection().cloned()
            {
                return Err(testing::PlayError::Rejected {
                    index,
                    step,
                    rejection,
                });
            }
            budget.record_pass()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND, MAIN_2};

    use super::play;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;

        play(
            &mut game,
            [p1.cast(bolt).targeting(p2).end_turn(), p2.end_turn()],
        )?;

        assert_eq!(game.player(p2).life(), 17);
        Ok(())
    }

    #[test]
    fn play_with_an_empty_mana_pool_reports_the_rejection() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;

        let error = play(
            &mut game,
            [p1.cast(bolt).targeting(p2).end_turn(), p2.end_turn()],
        )
        .unwrap_err();

        assert_eq!(
            error,
            PlayError::Rejected {
                index: 1,
                step: PlayStep::Act(*p1, actions::Action::ChooseTargets(vec![*p2])),
                rejection: actions::Rejected::CannotPay { spell: bolt.into() },
            }
        );
        Ok(())
    }

    #[test]
    fn a_negate_counters_a_bolt_and_the_turn_advances_to_the_next_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [negate] = game.player(p2).cards_in(HAND).exactly::<1>(NEGATE)?;

        game.play([
            p1.cast(bolt).targeting(p2).end_turn(),
            p2.cast(negate).targeting(bolt).end_turn(),
        ])?;

        assert_eq!(game.player(p2).life(), 20);
        assert!(game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT).is_ok());
        assert!(game.player(p2).cards_in(GRAVEYARD).exactly::<1>(NEGATE).is_ok());
        let turn = *game.turn();
        assert_eq!(turn.number, 2);
        assert_eq!(turn.active, p2.id);
        Ok(())
    }

    #[test]
    fn a_dispel_counters_a_cancel_so_the_bolt_resolves() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DISPEL]).mana_pool(mana![R, U]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [dispel] = game.player(p1).cards_in(HAND).exactly::<1>(DISPEL)?;
        let [cancel] = game.player(p2).cards_in(HAND).exactly::<1>(CANCEL)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.cast(dispel).targeting(cancel).end_turn(),
            p2.end_turn(),
        ])?;

        assert_eq!(game.player(p2).life(), 17);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(cancel));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(dispel));
        Ok(())
    }

    #[test]
    fn pass_to_advances_both_players_to_the_requested_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.pass_to(MAIN_2), p2.pass_to(MAIN_2)])?;

        let turn = *game.turn();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, p1.id);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
        Ok(())
    }

    #[test]
    fn end_phase_lands_on_the_first_step_of_the_next_phase() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_phase(), p2.end_phase()])?;

        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        Ok(())
    }

    #[test]
    fn end_current_step_moves_exactly_one_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_current_step(), p2.end_current_step()])?;

        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        Ok(())
    }

    #[test]
    fn activate_ability_with_only_on_a_forest_adds_green() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;

        game.play([p1.activate_ability(forest.ability())])?;

        assert_eq!(game.mana_pool(p1), mana![G]);
        Ok(())
    }

    #[test]
    fn activate_ability_with_only_on_a_tundra_is_ambiguous() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        let error = game.play([p1.activate_ability(tundra.ability())]).unwrap_err();

        assert_eq!(
            error,
            PlayError::AmbiguousAbility {
                index: 0,
                card: tundra.into(),
                count: 2,
            }
        );
        assert_eq!(game.mana_pool(p1), mana![]);
        Ok(())
    }

    #[test]
    fn activate_ability_with_mana_ability_on_a_tundra_adds_blue() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;

        game.play([
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::Blue)))
        ])?;

        assert_eq!(game.mana_pool(p1), mana![U]);
        Ok(())
    }
}
