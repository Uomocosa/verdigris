use crate::effects;

pub const fn nontoken(filter: effects::Filter) -> effects::Filter {
    effects::Filter {
        nontoken: true,
        ..filter
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;

    use super::nontoken;

    #[test]
    fn test_usage() {
        let filter = nontoken(CREATURE);
        assert!(filter.nontoken);
        assert_eq!(filter.kind, crate::effects::FilterKind::Creature);
    }
}
