#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Limit {
    At(u32),
    NoMaximum,
}

impl Limit {
    #[must_use]
    pub const fn allows(self, count: u32) -> bool {
        match self {
            Self::At(maximum) => count < maximum,
            Self::NoMaximum => true,
        }
    }

    #[must_use]
    pub fn excess(self, count: usize) -> usize {
        match self {
            Self::At(maximum) => {
                count.saturating_sub(usize::try_from(maximum).unwrap_or(usize::MAX))
            }
            Self::NoMaximum => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::Limit::At(7).allows(6));
        assert!(!rules::Limit::At(7).allows(7));
        assert_eq!(rules::Limit::At(7).excess(9), 2);
    }

    #[test]
    fn no_maximum_allows_any_count_with_no_excess() {
        assert!(rules::Limit::NoMaximum.allows(100));
        assert_eq!(rules::Limit::NoMaximum.excess(100), 0);
        assert_eq!(rules::Limit::At(7).excess(7), 0);
    }
}
