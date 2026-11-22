use crate::effects;

pub const fn attacking(filter: effects::Filter) -> effects::Filter {
    effects::Filter {
        attacking: true,
        ..filter
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;

    use super::attacking;

    #[test]
    fn test_usage() {
        let filter = attacking(CREATURE);
        assert!(filter.attacking);
        assert_eq!(filter.kind, crate::effects::FilterKind::Creature);
    }
}
