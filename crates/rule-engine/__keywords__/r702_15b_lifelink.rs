use crate::effects;
use crate::keywords;

/// 702.15b Damage dealt by a source with lifelink causes that source’s controller, or its owner if
/// it has no controller, to gain that much life (in addition to any other results that damage
/// causes). See rule 120.3.
#[must_use]
pub fn r702_15b_lifelink() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.15/lifelink");

const fn abilities() -> Vec<effects::Modification> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        assert_eq!(super::r702_15b_lifelink().id, super::ID);
        assert_eq!(super::r702_15b_lifelink().abilities(), super::abilities());
    }
}
