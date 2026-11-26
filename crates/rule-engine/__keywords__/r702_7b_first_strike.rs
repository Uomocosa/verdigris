use crate::effects;
use crate::keywords;

/// 702.7b If at least one attacking or blocking creature has first strike or double strike (see
/// rule 702.4) as the combat damage step begins, the only creatures that assign combat damage in
/// that step are those with first strike or double strike. After that step, instead of proceeding
/// to the end of combat step, the phase gets a second combat damage step. The only creatures that
/// assign combat damage in that step are the remaining attackers and blockers that had neither
/// first strike nor double strike as the first combat damage step began, as well as the remaining
/// attackers and blockers that currently have double strike. After that step, the phase proceeds to
/// the end of combat step.
#[must_use]
pub fn r702_7b_first_strike() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.7/first-strike");

const fn abilities() -> Vec<effects::Modification> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        assert_eq!(super::r702_7b_first_strike().id, super::ID);
        assert_eq!(
            super::r702_7b_first_strike().abilities(),
            super::abilities()
        );
    }
}
