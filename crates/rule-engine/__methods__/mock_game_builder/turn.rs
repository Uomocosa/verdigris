use crate::testing;
use crate::turns;

pub const fn turn(
    mut builder: testing::MockGameBuilder,
    number: u32,
    active: testing::Seat,
    phase: turns::Phase,
) -> testing::MockGameBuilder {
    builder.turn_number = number;
    builder.active = active;
    builder.phase = phase;
    builder
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::turns;

    use super::turn;

    #[test]
    fn test_usage() {
        let builder = turn(
            testing::MockGameBuilder::default(),
            2,
            testing::Seat::P2,
            turns::Phase::Combat,
        );
        let game = builder.build();
        let current = game.app.world().resource::<turns::Turn>();
        assert_eq!(current.number, 2);
        assert_eq!(current.phase, turns::Phase::Combat);
    }
}
