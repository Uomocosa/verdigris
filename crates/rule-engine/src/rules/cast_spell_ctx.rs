use bevy::prelude::Entity;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastSpellCtx {
    pub spell: Entity,
    pub player: Entity,
}

#[rustfmt::skip]
impl rules::traits::context::Context for CastSpellCtx {
    fn subject(&self) -> Entity { self.spell }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules::traits::context::Context;

    use super::CastSpellCtx;

    #[test]
    fn test_usage() {
        let spell = Entity::PLACEHOLDER;
        let player = Entity::from_raw_u32(1).unwrap();
        let context = CastSpellCtx { spell, player };
        assert_eq!(context.spell, spell);
        assert_eq!(context.player, player);
        assert_eq!(context.subject(), spell);
        assert_eq!(context.other(), None);
        assert_eq!(context.player(), player);
    }
}
