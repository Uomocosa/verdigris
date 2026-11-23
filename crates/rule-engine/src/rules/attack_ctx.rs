use bevy::prelude::Entity;
use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct AttackCtx(pub Entity);

#[rustfmt::skip]
impl rules::traits::pair_ctx::PairCtx for AttackCtx {
    fn subject(&self) -> Entity { **self }
    fn other(&self) -> Option<Entity> { None }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::pair_ctx::PairCtx;

    use super::AttackCtx;

    #[test]
    fn test_usage() {
        let attacker = Entity::PLACEHOLDER;
        let context = AttackCtx(attacker);
        assert_eq!(*context, attacker);
        assert_eq!(context.subject(), attacker);
        assert_eq!(context.other(), None);
    }
}
