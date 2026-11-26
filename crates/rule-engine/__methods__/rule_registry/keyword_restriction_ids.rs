use std::collections::HashSet;

use crate::effects;
use crate::keywords;
use crate::rules;

pub fn keyword_restriction_ids() -> HashSet<rules::RestrictionId> {
    [
        keywords::Deathtouch::abilities(),
        keywords::Defender::abilities(),
        keywords::DoubleStrike::abilities(),
        keywords::FirstStrike::abilities(),
        keywords::Flying::abilities(),
        keywords::Haste::abilities(),
        keywords::Hexproof::abilities(),
        keywords::Indestructible::abilities(),
        keywords::Infect::abilities(),
        keywords::Lifelink::abilities(),
        keywords::Reach::abilities(),
        keywords::Trample::abilities(),
        keywords::Vigilance::abilities(),
    ]
    .into_iter()
    .flatten()
    .filter_map(|modification| match modification {
        effects::Modification::Rule(change) => match change.change {
            rules::Change::Restrict { restriction, .. } => Some(restriction.id),
            _ => None,
        },
        effects::Modification::Characteristic(_) => None,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::keywords;
    use crate::rules;

    use super::keyword_restriction_ids;

    #[test]
    fn test_usage() {
        let ids = keyword_restriction_ids();
        assert!(ids.contains(&keywords::Flying::FLYING_BLOCKERS));
        assert!(ids.contains(&keywords::Defender::CANT_ATTACK));
        assert!(ids.contains(&keywords::Hexproof::CANT_BE_TARGETED));
        assert!(ids.contains(&keywords::Indestructible::CANT_BE_DESTROYED));
        assert!(
            ids.contains(&rules::RestrictionId("702.3b/defender")),
            "keyword ids carry the moved event restriction ids: {ids:?}"
        );
    }
}
