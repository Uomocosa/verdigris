use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::actions;
use crate::damage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatDamageChoice {
    pub player: Entity,
    pub needed: Vec<actions::RequiredCombatDamage>,
}

#[atomic_delegates]
impl CombatDamageChoice {
    #[must_use]
    pub(crate) fn assigned_damage(
        &self,
        player: Entity,
        assignments: &[actions::DamageAssignment],
    ) -> Option<Vec<damage::DamageDealt>> {
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        let player = Entity::from(p1);
        let source = Entity::from(p2);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p2, HAND).unwrap();

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [first, second].into_iter().collect(),
                trample_defender: None,
                lethal_by_target: BTreeMap::new(),
            }],
        };

        assert_eq!(choice.player, player);
        assert_eq!(
            choice.needed,
            vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [first, second].into_iter().collect(),
                trample_defender: None,
                lethal_by_target: BTreeMap::new(),
            }]
        );
    }
}
