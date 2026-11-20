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
    use bevy::prelude::World;

    use crate::actions;
    use crate::damage;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let player = world.spawn_empty().id();
        let source = world.spawn_empty().id();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [first, second].into_iter().collect(),
            }],
        };

        assert_eq!(
            choice.assigned_damage(
                player,
                &[
                    actions::DamageAssignment {
                        target: first,
                        amount: 1
                    },
                    actions::DamageAssignment {
                        target: second,
                        amount: 1
                    },
                ]
            ),
            Some(vec![
                damage::DamageDealt {
                    source,
                    target: first,
                    amount: 1
                },
                damage::DamageDealt {
                    source,
                    target: second,
                    amount: 1
                },
            ])
        );
    }
}
