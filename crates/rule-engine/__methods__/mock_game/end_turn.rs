use crate::testing;
use crate::turns;

const MAX_PASS_ROUNDS_BEFORE_A_NEW_TURN: u32 = 16;

pub fn end_turn(game: &mut testing::MockGame) {
    let number = game.app.world().resource::<turns::Turn>().number;
    for _ in 0..MAX_PASS_ROUNDS_BEFORE_A_NEW_TURN {
        if game.app.world().resource::<turns::Turn>().number != number {
            return;
        }
        game.pass_priority_all();
    }
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::turns;

    use super::end_turn;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder().build();
        let p2 = game.p2;

        end_turn(&mut game);

        let turn = *game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 2);
        assert_eq!(turn.active, p2);
    }
}
