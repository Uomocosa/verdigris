use crate::effects;
use crate::zones;

pub const fn from_zone(zone: zones::Zone) -> effects::ZoneRef {
    effects::ZoneRef {
        zone,
        of: effects::PlayerRef::Any,
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::{BATTLEFIELD, HAND};

    use super::from_zone;

    #[test]
    fn test_usage() {
        assert_eq!(
            from_zone(HAND),
            effects::ZoneRef {
                zone: HAND,
                of: effects::PlayerRef::Any,
            }
        );
        assert_eq!(
            effects::ZoneRef::from(BATTLEFIELD),
            effects::ZoneRef {
                zone: BATTLEFIELD,
                of: effects::PlayerRef::Any,
            }
        );
    }
}
