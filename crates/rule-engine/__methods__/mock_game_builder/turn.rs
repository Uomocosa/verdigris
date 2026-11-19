use crate::testing;
use crate::turns;

pub const fn turn(
    mut builder: testing::MockGameBuilder,
    number: u32,
    active: testing::Seat,
    step: turns::Step,
) -> testing::MockGameBuilder {
    builder.turn_number = number;
    builder.active = active;
    builder.step = step;
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
            turns::Step::CombatDamage,
        );
        let game = builder.build();
        let current = game.app.world().resource::<turns::Turn>();
        assert_eq!(current.number, 2);
        assert_eq!(current.step, turns::Step::CombatDamage);
    }
}
