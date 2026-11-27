use crate::PLAYER;
use crate::effects;
use crate::rules;

pub const fn player(who: rules::Players) -> effects::Filter {
    effects::Filter {
        kind: effects::FilterKind::Player,
        controlled_by: Some(who),
        ..PLAYER
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;

    use super::player;

    #[test]
    fn test_usage() {
        let filter = player(YOU);
        assert_eq!(filter.kind, effects::FilterKind::Player);
        assert_eq!(filter.controlled_by, Some(YOU));
    }
}
