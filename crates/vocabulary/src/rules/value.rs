use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Value(pub i32);

impl Value {
    #[must_use]
    pub const fn or_more(self) -> rules::Comparison {
        let Self(inner) = self;
        rules::Comparison::OrMore(inner)
    }

    #[must_use]
    pub const fn or_less(self) -> rules::Comparison {
        let Self(inner) = self;
        rules::Comparison::OrLess(inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::Value;

    #[test]
    fn test_usage() {
        assert_eq!(Value(2).or_more(), rules::Comparison::OrMore(2));
        assert_eq!(Value(2).or_less(), rules::Comparison::OrLess(2));
        assert!(Value(2).or_more().holds(3));
        assert!(!Value(2).or_more().holds(1));
    }
}
