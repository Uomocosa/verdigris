#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Answer {
    TapsWhenAttacking(bool),
    IgnoresSummoningSickness(bool),
    MustAttack(bool),
    MustBlock(bool),
}

impl Answer {
    #[must_use]
    pub(crate) const fn taps_when_attacking(self) -> Option<bool> {
        match self {
            Self::TapsWhenAttacking(taps) => Some(taps),
            Self::IgnoresSummoningSickness(_) | Self::MustAttack(_) | Self::MustBlock(_) => None,
        }
    }

    #[must_use]
    pub(crate) const fn ignores_summoning_sickness(self) -> Option<bool> {
        match self {
            Self::IgnoresSummoningSickness(ignores) => Some(ignores),
            Self::TapsWhenAttacking(_) | Self::MustAttack(_) | Self::MustBlock(_) => None,
        }
    }

    #[must_use]
    pub(crate) const fn must_attack(self) -> Option<bool> {
        match self {
            Self::MustAttack(must) => Some(must),
            Self::TapsWhenAttacking(_) | Self::IgnoresSummoningSickness(_) | Self::MustBlock(_) => {
                None
            }
        }
    }

    #[must_use]
    pub(crate) const fn must_block(self) -> Option<bool> {
        match self {
            Self::MustBlock(must) => Some(must),
            Self::TapsWhenAttacking(_)
            | Self::IgnoresSummoningSickness(_)
            | Self::MustAttack(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::Answer::TapsWhenAttacking(true).taps_when_attacking(),
            Some(true)
        );
        assert_eq!(
            rules::Answer::IgnoresSummoningSickness(true).taps_when_attacking(),
            None
        );
        assert_eq!(
            rules::Answer::IgnoresSummoningSickness(false).ignores_summoning_sickness(),
            Some(false)
        );
        assert_eq!(
            rules::Answer::TapsWhenAttacking(false).ignores_summoning_sickness(),
            None
        );
        assert_eq!(rules::Answer::MustAttack(true).must_attack(), Some(true));
        assert_eq!(
            rules::Answer::IgnoresSummoningSickness(true).must_attack(),
            None
        );
        assert_eq!(rules::Answer::MustBlock(false).must_block(), Some(false));
        assert_eq!(rules::Answer::TapsWhenAttacking(true).must_block(), None);
    }
}
