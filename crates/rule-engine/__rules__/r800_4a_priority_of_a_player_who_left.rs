use crate::GameMut;

/// 800.4a When a player leaves the game, all objects (see rule 109) owned by that player leave the
/// game and any effects which give that player control of any objects or players end. Then, if that
/// player controlled any objects on the stack not represented by cards, those objects cease to
/// exist. Then, if there are any objects still controlled by that player, those objects are exiled.
/// This is not a state-based action. It happens as soon as the player leaves the game. If the
/// player who left the game had priority at the time they left, priority passes to the next player
/// in turn order who’s still in the game.
///
/// # Notes
///
/// Only the last sentence is implemented; objects and effects of a player who left are not handled
/// yet.
pub fn r800_4a_priority_of_a_player_who_left(mut game: GameMut) {
    let Some(holder) = game.priority_holder() else {
        return;
    };
    if holder.is_still_in_the_game() {
        return;
    }
    if let Some(next) = holder.next_in_turn_order() {
        game.pass_priority_to(next.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn priority_passes_on_when_its_holder_concedes() {
        let (mut game, [p1, p2, _p3]) = MockGame::new([P1, P2, P3]);

        game.act(p1, actions::Action::Concede);

        assert_eq!(game.priority_player(), Some(p2.id));
    }

    #[test]
    fn priority_passes_on_when_its_holder_loses_to_a_state_based_action() -> TestResult {
        let (mut game, [p1, _p2, p3]) = MockGame::new([P1, P2.life(0), P3]);

        game.play([p1.pass_priority()])?;

        assert_eq!(game.priority_player(), Some(p3.id));
        Ok(())
    }
}
