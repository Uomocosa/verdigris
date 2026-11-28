#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    OrMore(i32),
    OrLess(i32),
    Exactly(i32),
    GreaterThan(i32),
    LessThan(i32),
}

impl Comparison {
    #[must_use]
    pub const fn holds(self, value: i32) -> bool {
        match self {
            Self::OrMore(threshold) => value >= threshold,
            Self::OrLess(threshold) => value <= threshold,
            Self::Exactly(threshold) => value == threshold,
            Self::GreaterThan(threshold) => value > threshold,
            Self::LessThan(threshold) => value < threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Comparison;

    #[test]
    fn test_usage() {
        assert!(Comparison::OrMore(2).holds(2));
        assert!(!Comparison::OrMore(2).holds(1));
        assert!(Comparison::GreaterThan(2).holds(3));
        assert!(Comparison::Exactly(2).holds(2));
        assert!(Comparison::OrLess(2).holds(1));
        assert!(Comparison::LessThan(2).holds(1));
    }
}
