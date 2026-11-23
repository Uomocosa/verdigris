use bevy::prelude::Entity;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayLandCtx {
    pub land: Entity,
    pub player: Entity,
}

#[rustfmt::skip]
impl rules::traits::context::Context for PlayLandCtx {
    fn subject(&self) -> Entity { self.land }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::context::Context;

    use super::PlayLandCtx;

    #[test]
    fn test_usage() {
        let land = Entity::PLACEHOLDER;
        let player = Entity::from_raw_u32(1).unwrap();
        let context = PlayLandCtx { land, player };
        assert_eq!(context.land, land);
        assert_eq!(context.player, player);
        assert_eq!(context.subject(), land);
        assert_eq!(context.other(), None);
        assert_eq!(context.player(), player);
    }
}
