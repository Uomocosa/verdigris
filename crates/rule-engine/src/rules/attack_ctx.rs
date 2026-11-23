use bevy::prelude::Entity;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttackCtx {
    pub attacker: Entity,
    pub player: Entity,
}

#[rustfmt::skip]
impl rules::traits::pair_ctx::PairCtx for AttackCtx {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::pair_ctx::PairCtx;

    use super::AttackCtx;

    #[test]
    fn test_usage() {
        let attacker = Entity::PLACEHOLDER;
        let player = Entity::from_raw_u32(1).unwrap();
        let context = AttackCtx { attacker, player };
        assert_eq!(context.attacker, attacker);
        assert_eq!(context.player, player);
        assert_eq!(context.subject(), attacker);
        assert_eq!(context.other(), None);
        assert_eq!(context.player(), player);
    }
}
