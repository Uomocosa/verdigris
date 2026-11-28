use std::collections::HashMap;

use bevy::prelude::Component;
use derive_more::Deref;

use crate::counters;

#[derive(Component, Debug, Clone, Default, PartialEq, Eq, Deref)]
pub struct Counters(HashMap<counters::CounterId, u32>);

impl Counters {
    #[must_use]
    pub fn get(&self, kind: counters::CounterId) -> u32 {
        let Self(counts) = self;
        counts.get(&kind).copied().unwrap_or(0)
    }

    pub fn add(&mut self, kind: counters::CounterId, amount: u32) {
        let Self(counts) = self;
        let total = counts.get(&kind).copied().unwrap_or(0).saturating_add(amount);
        counts.insert(kind, total);
    }

    pub fn remove(&mut self, kind: counters::CounterId, amount: u32) {
        let Self(counts) = self;
        let remaining = counts.get(&kind).copied().unwrap_or(0).saturating_sub(amount);
        if remaining == 0 {
            counts.remove(&kind);
        } else {
            counts.insert(kind, remaining);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::counters;

    use super::Counters;

    #[test]
    fn test_usage() {
        let mut held = Counters::default();
        assert_eq!(held.get(counters::POISON), 0);
        held.add(counters::POISON, 3);
        assert_eq!(held.get(counters::POISON), 3);
        held.remove(counters::POISON, 1);
        assert_eq!(held.get(counters::POISON), 2);
        held.remove(counters::POISON, 5);
        assert_eq!(held.get(counters::POISON), 0);
        assert_eq!(
            3 * counters::POISON,
            counters::CounterAmount {
                count: 3,
                kind: counters::POISON
            }
        );
    }
}
