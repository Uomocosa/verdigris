use std::collections::HashSet;

use crate::effects;
use crate::keywords;
use crate::rules;

pub fn keyword_restriction_ids() -> HashSet<rules::RestrictionId> {
    [
        keywords::deathtouch().abilities(),
        keywords::defender().abilities(),
        keywords::double_strike().abilities(),
        keywords::first_strike().abilities(),
        keywords::flying().abilities(),
        keywords::haste().abilities(),
        keywords::hexproof().abilities(),
        keywords::indestructible().abilities(),
        keywords::infect().abilities(),
        keywords::lifelink().abilities(),
        keywords::reach().abilities(),
        keywords::trample().abilities(),
        keywords::vigilance().abilities(),
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
        assert!(ids.contains(&keywords::flying::BLOCKERS));
        assert!(ids.contains(&keywords::defender::CANT_ATTACK));
        assert!(ids.contains(&keywords::hexproof::CANT_BE_TARGETED));
        assert!(ids.contains(&keywords::indestructible::CANT_BE_DESTROYED));
        assert!(
            ids.contains(&rules::RestrictionId("702.3b/defender")),
            "keyword ids carry the moved event restriction ids: {ids:?}"
        );
    }
}
