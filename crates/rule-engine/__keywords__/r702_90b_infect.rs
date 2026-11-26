use crate::effects;
use crate::keywords;

/// 702.90b Damage dealt to a player by a source with infect doesn’t cause that player to lose life.
/// Rather, it causes that source’s controller to give the player that many poison counters. See
/// rule 120.3.
#[must_use]
pub fn r702_90b_infect() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.90/infect");

const fn abilities() -> Vec<effects::Modification> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        assert_eq!(super::r702_90b_infect().id, super::ID);
        assert_eq!(super::r702_90b_infect().abilities(), super::abilities());
    }
}
