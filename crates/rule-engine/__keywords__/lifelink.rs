use crate::effects;
use crate::keywords;

/// 702.15b Damage dealt by a source with lifelink causes that source’s controller, or its owner if it has no controller, to gain that much life (in addition to any other results that damage causes). See rule 120.3.
pub struct Lifelink;

impl Lifelink {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.15/lifelink");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl Lifelink {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::Lifelink;

    #[test]
    fn test_usage() {
        assert_eq!(Lifelink::keyword().id, Lifelink::ID);
        assert_eq!(Lifelink::keyword().abilities(), Lifelink::abilities());
    }
}
