use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut)]
pub struct Timestamp(pub u64);

impl Timestamp {
    #[must_use]
    pub fn advance(&mut self) -> u64 {
        let next = **self;
        **self = self.wrapping_add(1);
        next
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;

    #[test]
    fn test_usage() {
        let mut timestamp = effects::Timestamp::default();
        assert_eq!(timestamp.advance(), 0);
        assert_eq!(timestamp.advance(), 1);
        assert_eq!(*timestamp, 2);
    }
}
