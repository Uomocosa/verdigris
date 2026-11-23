use bevy::prelude::Entity;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockCtx {
    pub blocker: Entity,
    pub attacker: Entity,
}

#[rustfmt::skip]
impl rules::traits::pair_ctx::PairCtx for BlockCtx {
    fn subject(&self) -> Entity { self.blocker }
    fn other(&self) -> Option<Entity> { Some(self.attacker) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::pair_ctx::PairCtx;

    use super::BlockCtx;

    #[test]
    fn test_usage() {
        let blocker = Entity::PLACEHOLDER;
        let attacker = Entity::from_raw_u32(1).unwrap();
        let context = BlockCtx { blocker, attacker };
        assert_eq!(context.subject(), blocker);
        assert_eq!(context.other(), Some(attacker));
    }
}
