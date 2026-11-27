use crate::effects;
use crate::rules;

pub const fn of(zone_ref: effects::ZoneRef, player: rules::Players) -> effects::ZoneRef {
    effects::ZoneRef {
        zone: zone_ref.zone,
        of: player,
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::YOU;
    use crate::effects;
    use crate::rules;

    use super::of;

    #[test]
    fn test_usage() {
        let hand = effects::ZoneRef {
            zone: HAND,
            of: rules::Players::All,
        };
        assert_eq!(of(hand, YOU).of, YOU);
    }
}
