use crate::testing;

pub fn play(
    game: &mut testing::MockGame,
    casts: impl IntoIterator<Item = testing::Cast>,
) -> Result<(), testing::PlayError> {
    for (index, step) in casts.into_iter().flat_map(|cast| cast.steps()).enumerate() {
        let before = game.rejection_count();
        match &step {
            testing::Step::Act(player, action) => game.act(*player, action.clone()),
            testing::Step::PassPriorityAll => game.pass_priority_all(),
        }
        let after = game.rejection_count();
        if after > before
            && let Some(rejection) = game.last_rejection().cloned()
        {
            return Err(testing::PlayError::Rejected {
                index,
                step,
                rejection,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::mana;
    use crate::testing::traits::Caster;
    use crate::testing::*;

    use super::play;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();

        play(&mut game, [p1.cast(bolt).targeting(p2).resolve()]).unwrap();

        assert_eq!(game.player(p2).life(), 17);
    }

    #[test]
    fn play_with_an_empty_mana_pool_reports_the_rejection() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();

        let error = play(&mut game, [p1.cast(bolt).targeting(p2).resolve()]).unwrap_err();

        assert_eq!(
            error,
            PlayError::Rejected {
                index: 1,
                step: Step::Act(p1, actions::Action::ChooseTargets(vec![p2])),
                rejection: actions::Rejected::CannotPay { spell: bolt },
            }
        );
    }
}
