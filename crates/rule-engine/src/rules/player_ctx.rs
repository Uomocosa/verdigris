use bevy::prelude::Entity;
use derive_more::Deref;
use derive_more::From;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct PlayerCtx(pub Entity);

#[rustfmt::skip]
impl rules::traits::context::Context for PlayerCtx {
    fn subject(&self) -> Entity { **self }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { **self }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::context::Context;

    use super::PlayerCtx;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let context = PlayerCtx(player);
        assert_eq!(*context, player);
        assert_eq!(context.subject(), player);
        assert_eq!(context.other(), None);
        assert_eq!(context.player(), player);
    }
}
