use crate::effects;
use crate::rules;

pub const fn controlled_by(filter: effects::Filter, player: rules::Players) -> effects::Filter {
    effects::Filter {
        controlled_by: Some(player),
        ..filter
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::OPPONENT;
    use crate::effects;

    use super::controlled_by;

    #[test]
    fn test_usage() {
        let controlled = controlled_by(CREATURE, OPPONENT);
        assert_eq!(controlled.controlled_by, Some(OPPONENT));
        assert_eq!(controlled.kind, effects::FilterKind::Creature);
    }
}
