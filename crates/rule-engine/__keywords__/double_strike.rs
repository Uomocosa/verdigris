use crate::effects;
use crate::keywords;

/// 702.4b If at least one attacking or blocking creature has first strike (see rule 702.7) or double strike as the combat damage step begins, the only creatures that assign combat damage in that step are those with first strike or double strike. After that step, instead of proceeding to the end of combat step, the phase gets a second combat damage step. The only creatures that assign combat damage in that step are the remaining attackers and blockers that had neither first strike nor double strike as the first combat damage step began, as well as the remaining attackers and blockers that currently have double strike. After that step, the phase proceeds to the end of combat step.
pub struct DoubleStrike;

impl DoubleStrike {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.4/double-strike");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }
}

#[rustfmt::skip]
impl DoubleStrike {
    #[must_use]
    pub const fn abilities() -> Vec<effects::Modification> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::DoubleStrike;

    #[test]
    fn test_usage() {
        assert_eq!(DoubleStrike::keyword().id, DoubleStrike::ID);
        assert_eq!(
            DoubleStrike::keyword().abilities(),
            DoubleStrike::abilities()
        );
    }
}
