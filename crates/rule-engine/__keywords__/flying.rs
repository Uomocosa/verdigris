use crate::effects;
use crate::keywords;

/// 702.9b A creature with flying can’t be blocked except by creatures with flying and/or reach. A creature with flying can block a creature with or without flying. (See rule 509, “Declare Blockers Step,” and rule 702.17, “Reach.”)
pub struct Flying;

impl Flying {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.9/flying");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl Flying {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::Flying;

    #[test]
    fn test_usage() {
        assert_eq!(Flying::keyword().id, Flying::ID);
        assert_eq!(Flying::keyword().abilities(), Flying::abilities());
    }
}
