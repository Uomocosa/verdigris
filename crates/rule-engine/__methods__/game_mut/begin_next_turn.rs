use crate::GameMut;
use crate::players;
use crate::turns;

pub fn begin_next_turn<'w, 's>(game: &mut GameMut<'w, 's>) -> Option<players::PlayerView<'w, 's>> {
    let next = game.players.view(game.turn_state.turn.active).next_in_turn_order()?;
    let number = game.turn_state.turn.number.saturating_add(1);
    *game.turn_state = turns::TurnState::from(turns::Turn {
        number,
        active: next.id,
        step: turns::Step::Untap,
    });
    game.turn_state.priority.give_to(next.id);
    Some(next)
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::GameMut;
    use crate::testing::*;
    use crate::turns;

    use super::begin_next_turn;

    #[test]
    fn test_usage() {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |mut answers: GameMut| {
                let next = begin_next_turn(&mut answers);
                assert_eq!(next.map(|player| player.id), Some(p2.id));
                assert_eq!(answers.step(), turns::Step::Untap);
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p2.id)
                );
            })
            .unwrap();
    }
}
