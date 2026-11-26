use crate::effects;
use crate::keywords;

/// 702.17a Reach is a static ability.
pub struct Reach;

impl Reach {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.17/reach");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl Reach {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::Reach;

    #[test]
    fn test_usage() {
        assert_eq!(Reach::keyword().id, Reach::ID);
        assert_eq!(Reach::keyword().abilities(), Reach::abilities());
        assert_eq!(Reach::abilities(), Vec::new());
    }
}
