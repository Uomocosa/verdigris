use std::collections::BTreeSet;

use crate::testing;

#[must_use]
pub const fn pick(count: u32) -> testing::ExpectedTargets {
    testing::ExpectedTargets {
        count,
        legal: BTreeSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::pick;

    #[test]
    fn test_usage() {
        let targets = pick(2);
        assert_eq!(targets.count, 2);
        assert!(targets.legal.is_empty());
    }
}
