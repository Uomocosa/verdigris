use crate::effects;
use crate::keywords;

/// 702.90b Damage dealt to a player by a source with infect doesn’t cause that player to lose life. Rather, it causes that source’s controller to give the player that many poison counters. See rule 120.3.
pub struct Infect;

impl Infect {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.90/infect");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl Infect {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::Infect;

    #[test]
    fn test_usage() {
        assert_eq!(Infect::keyword().id, Infect::ID);
        assert_eq!(Infect::keyword().abilities(), Infect::abilities());
    }
}
