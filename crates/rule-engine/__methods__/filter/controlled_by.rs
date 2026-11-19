use crate::effects;

pub const fn controlled_by(filter: effects::Filter, player: effects::PlayerRef) -> effects::Filter {
    effects::Filter {
        controlled_by: Some(player),
        ..filter
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;

    use super::controlled_by;

    #[test]
    fn test_usage() {
        let controlled = controlled_by(CREATURE, effects::PlayerRef::Opponent);
        assert_eq!(controlled.controlled_by, Some(effects::PlayerRef::Opponent));
        assert_eq!(controlled.kind, effects::FilterKind::Creature);
    }
}
