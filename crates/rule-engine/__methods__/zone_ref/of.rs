use crate::effects;

pub const fn of(zone_ref: effects::ZoneRef, player: effects::PlayerRef) -> effects::ZoneRef {
    effects::ZoneRef {
        zone: zone_ref.zone,
        of: player,
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::effects;

    use super::of;

    #[test]
    fn test_usage() {
        let hand = effects::ZoneRef {
            zone: HAND,
            of: effects::PlayerRef::Any,
        };
        assert_eq!(
            of(hand, effects::PlayerRef::You).of,
            effects::PlayerRef::You
        );
    }
}
