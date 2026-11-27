use crate::effects;
use crate::rules;

pub fn of(zone_ref: effects::ZoneRef, player: impl Into<rules::Players>) -> effects::ZoneRef {
    effects::ZoneRef {
        zone: zone_ref.zone,
        of: player.into(),
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
        assert_eq!(
            of(hand, YOU),
            effects::ZoneRef {
                zone: HAND,
                of: rules::Players::You,
            }
        );
    }
}
