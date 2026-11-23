use bevy::prelude::Entity;

pub trait PairCtx {
    fn subject(&self) -> Entity;
    fn other(&self) -> Option<Entity>;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::pair_ctx::PairCtx;

    #[test]
    fn test_usage() {
        let attacker = Entity::PLACEHOLDER;
        let attack = rules::AttackCtx(attacker);
        assert_eq!(attack.subject(), attacker);
        assert_eq!(attack.other(), None);
        let block = rules::BlockCtx {
            blocker: Entity::from_raw_u32(1).unwrap(),
            attacker,
        };
        assert_eq!(block.subject(), Entity::from_raw_u32(1).unwrap());
        assert_eq!(block.other(), Some(attacker));
    }
}
