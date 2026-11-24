use bevy::prelude::Entity;

use crate::players;

pub trait Context {
    fn subject(&self) -> Entity;
    fn other(&self) -> Option<Entity>;
    fn player(&self) -> players::PlayerId;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;

    use crate::rules;
    use crate::rules::traits::context::Context;

    #[test]
    fn test_usage() {
        let attacker = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let attack = rules::DeclareAttacker { attacker, player };
        assert_eq!(attack.subject(), attacker);
        assert_eq!(attack.other(), None);
        assert_eq!(attack.player(), player);
        let block = rules::DeclareBlocker {
            blocker: Entity::from_raw_u32(1).unwrap(),
            attacker,
            player,
        };
        assert_eq!(block.subject(), Entity::from_raw_u32(1).unwrap());
        assert_eq!(block.other(), Some(attacker));
        assert_eq!(block.player(), player);
    }
}
