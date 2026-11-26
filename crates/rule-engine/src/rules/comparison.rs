#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    AtLeast,
    AtMost,
    EqualTo,
    GreaterThan,
    LessThan,
}

impl Comparison {
    #[must_use]
    pub fn holds<V: PartialOrd>(self, value: &V, threshold: &V) -> bool {
        match self {
            Self::AtLeast => value >= threshold,
            Self::AtMost => value <= threshold,
            Self::EqualTo => value == threshold,
            Self::GreaterThan => value > threshold,
            Self::LessThan => value < threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Comparison;

    #[test]
    fn test_usage() {
        assert!(Comparison::AtLeast.holds(&2, &2));
        assert!(!Comparison::AtLeast.holds(&1, &2));
        assert!(Comparison::GreaterThan.holds(&3, &2));
        assert!(Comparison::EqualTo.holds(&2, &2));
        assert!(Comparison::AtMost.holds(&1, &2));
        assert!(Comparison::LessThan.holds(&1, &2));
    }
}
