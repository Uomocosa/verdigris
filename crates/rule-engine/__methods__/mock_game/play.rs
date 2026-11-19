use crate::actions;
use crate::testing;
use crate::turns;

const MAX_PASSES_PER_PLAY: u32 = 256;

pub fn play(
    game: &mut testing::MockGame,
    actions: impl IntoIterator<Item = testing::TestPlayerActions>,
) -> Result<(), testing::PlayError> {
    let mut auto_passes: Vec<testing::ActiveAutoPass> = Vec::new();
    let mut budget = testing::PassBudget::of(MAX_PASSES_PER_PLAY);
    for (index, step) in actions
        .into_iter()
        .flat_map(|action| action.steps)
        .enumerate()
    {
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
            let holder = game.app.world().resource::<turns::Priority>().player;
            let Some(position) = auto_passes
                .iter()
                .position(|auto_pass| auto_pass.player == holder)
            else {
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
            game.act(holder, actions::Action::PassPriority);
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
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{GRAVEYARD, HAND, MAIN_2};

    use super::play;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();

        play(
            &mut game,
            [p1.cast(bolt).targeting(p2).end_turn(), p2.end_turn()],
        )
        .unwrap();

        assert_eq!(game.player(p2).life(), 17);
    }

    #[test]
    fn play_with_an_empty_mana_pool_reports_the_rejection() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();

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
                rejection: actions::Rejected::CannotPay { spell: bolt },
            }
        );
    }

    #[test]
    fn a_negate_counters_a_bolt_and_the_turn_advances_to_the_next_player() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let negate = game.player(p2).cards(HAND, NEGATE).only().unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).end_turn(),
            p2.cast(negate).targeting(bolt).end_turn(),
        ])
        .unwrap();

        assert_eq!(game.player(p2).life(), 20);
        assert!(
            game.player(p1)
                .cards(GRAVEYARD, LIGHTNING_BOLT)
                .only()
                .is_ok()
        );
        assert!(game.player(p2).cards(GRAVEYARD, NEGATE).only().is_ok());
        let turn = *game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 2);
        assert_eq!(turn.active, p2);
    }

    #[test]
    fn a_dispel_counters_a_cancel_so_the_bolt_resolves() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DISPEL]).mana_pool(mana![R, U]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let dispel = game.player(p1).cards(HAND, DISPEL).only().unwrap();
        let cancel = game.player(p2).cards(HAND, CANCEL).only().unwrap();

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(cancel).targeting(bolt).pass_priority(),
            p1.cast(dispel).targeting(cancel).end_turn(),
            p2.end_turn(),
        ])
        .unwrap();

        assert_eq!(game.player(p2).life(), 17);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(cancel), GRAVEYARD);
        assert_eq!(game.zone_of(dispel), GRAVEYARD);
    }

    #[test]
    fn pass_to_advances_both_players_to_the_requested_step() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.pass_to(MAIN_2), p2.pass_to(MAIN_2)]).unwrap();

        let turn = *game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, p1);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
    }

    #[test]
    fn end_phase_lands_on_the_first_step_of_the_next_phase() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_phase(), p2.end_phase()]).unwrap();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::BeginningOfCombat
        );
    }

    #[test]
    fn end_current_step_moves_exactly_one_step() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_current_step(), p2.end_current_step()])
            .unwrap();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::BeginningOfCombat
        );
    }
}
