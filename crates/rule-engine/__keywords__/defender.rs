use crate::effects;
use crate::keywords;

/// 702.3a Defender is a static ability.
pub struct Defender;

impl Defender {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.3/defender");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl Defender {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::Defender;

    #[test]
    fn test_usage() {
        assert_eq!(Defender::keyword().id, Defender::ID);
        assert_eq!(Defender::keyword().abilities(), Defender::abilities());
    }
}
