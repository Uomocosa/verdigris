use bevy::ecs::system::SystemState;
use bevy::prelude::Res;

use crate::actions;
use crate::rules;
use crate::testing;

pub fn read_rules<R>(
    game: &mut testing::MockGame,
    with: impl FnOnce(&rules::Rules, &actions::LegalActions) -> R,
) -> Option<R> {
    let mut states =
        SystemState::<(rules::Rules, Res<actions::LegalActions>)>::new(game.app.world_mut());
    let (answers, legal) = states.get(game.app.world()).ok()?;
    Some(with(&answers, &legal))
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [_forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        assert_eq!(game.read_rules(|_answers, legal| legal.player), Some(None));
        game.play([p1.pass_priority()])?;
        assert_eq!(
            game.read_rules(|answers, _legal| answers
                .players_in_game(crate::rules::Players::All, Some(p1.id))
                .len()),
            Some(2)
        );
        Ok(())
    }
}
