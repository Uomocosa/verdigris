#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Answer {
    TapsWhenAttacking(bool),
    IgnoresSummoningSickness(bool),
}

impl Answer {
    #[must_use]
    pub(crate) const fn taps_when_attacking(self) -> Option<bool> {
        match self {
            Self::TapsWhenAttacking(taps) => Some(taps),
            Self::IgnoresSummoningSickness(_) => None,
        }
    }

    #[must_use]
    pub(crate) const fn ignores_summoning_sickness(self) -> Option<bool> {
        match self {
            Self::IgnoresSummoningSickness(ignores) => Some(ignores),
            Self::TapsWhenAttacking(_) => None,
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
    }
}
