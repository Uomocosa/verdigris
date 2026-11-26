use crate::PLAYER;
use crate::effects;

pub const fn player(who: effects::PlayerRef) -> effects::Filter {
    effects::Filter {
        kind: effects::FilterKind::Player,
        controlled_by: Some(who),
        ..PLAYER
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;

    use super::player;

    #[test]
    fn test_usage() {
        let filter = player(effects::PlayerRef::You);
        assert_eq!(filter.kind, effects::FilterKind::Player);
        assert_eq!(filter.controlled_by, Some(effects::PlayerRef::You));
    }
}
