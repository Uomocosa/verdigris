use crate::effects;
use crate::rules;

pub fn player(who: impl Into<rules::Players>) -> effects::Filter {
    effects::Filter {
        kind: effects::FilterKind::Player,
        controlled_by: Some(who.into()),
        excluded_color: None,
        attacking: false,
        nontoken: false,
        object: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;
    use crate::rules;

    use super::player;

    #[test]
    fn test_usage() {
        let filter = player(YOU);
        assert_eq!(filter.kind, effects::FilterKind::Player);
        assert_eq!(filter.controlled_by, Some(rules::Players::You));
    }
}
