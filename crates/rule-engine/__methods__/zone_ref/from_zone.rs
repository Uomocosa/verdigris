use crate::effects;
use crate::rules;
use crate::zones;

pub const fn from_zone(zone: zones::Zone) -> effects::ZoneRef {
    effects::ZoneRef {
        zone,
        of: rules::Players::All,
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::rules;
    use crate::{BATTLEFIELD, HAND};

    use super::from_zone;

    #[test]
    fn test_usage() {
        assert_eq!(
            from_zone(HAND),
            effects::ZoneRef {
                zone: HAND,
                of: rules::Players::All,
            }
        );
        assert_eq!(
            effects::ZoneRef::from(BATTLEFIELD),
            effects::ZoneRef {
                zone: BATTLEFIELD,
                of: rules::Players::All,
            }
        );
    }
}
