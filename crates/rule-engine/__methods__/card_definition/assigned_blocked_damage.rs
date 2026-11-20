use bevy::prelude::{Entity, Query};

use crate::actions;
use crate::cards;
use crate::damage;
use crate::effects;

pub fn assigned_blocked_damage(
    definition: &cards::CardDefinition,
    source: Entity,
    amount: u32,
    defending: Entity,
    blockers: &Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Blocking,
        Option<&damage::Damage>,
    )>,
    modifiers: &Query<&effects::PowerToughnessModifier>,
    needed: &mut Vec<actions::RequiredCombatDamage>,
) -> Vec<damage::DamageDealt> {
    let mut writes = Vec::new();
    let mut blockers_of: Vec<(Entity, u32)> = Vec::new();
    for (blocker, blocker_definition, blocking, marked) in blockers.iter() {
        if **blocking != source {
            continue;
        }
        let lethal = blocker_definition
            .effective_power_toughness(modifiers.get(blocker).ok())
            .map_or(0, |power_toughness| {
                u32::try_from(power_toughness.toughness).unwrap_or_default()
            })
            .saturating_sub(marked.map_or(0, |damage| **damage));
        blockers_of.push((blocker, lethal));
        let Some(blocker_power_toughness) =
            blocker_definition.effective_power_toughness(modifiers.get(blocker).ok())
        else {
            continue;
        };
        if blocker_power_toughness.power <= 0 {
            continue;
        }
        let Ok(blocker_amount) = u32::try_from(blocker_power_toughness.power) else {
            continue;
        };
        writes.push(damage::DamageDealt {
            source: blocker,
            target: source,
            amount: blocker_amount,
        });
    }
    match blockers_of.as_slice() {
        [(blocker, lethal)] => {
            let into_blocker = if definition.keywords.contains(&cards::Keyword::Trample) {
                amount.min(*lethal)
            } else {
                amount
            };
            if into_blocker > 0 {
                writes.push(damage::DamageDealt {
                    source,
                    target: *blocker,
                    amount: into_blocker,
                });
            }
            let into_defender = amount.saturating_sub(into_blocker);
            if into_defender > 0 {
                writes.push(damage::DamageDealt {
                    source,
                    target: defending,
                    amount: into_defender,
                });
            }
        }
        [] if definition.keywords.contains(&cards::Keyword::Trample) => {
            writes.push(damage::DamageDealt {
                source,
                target: defending,
                amount,
            });
        }
        [] => {}
        _ => needed.push(actions::RequiredCombatDamage {
            source,
            amount,
            legal: blockers_of.iter().map(|(blocker, _)| *blocker).collect(),
        }),
    }
    writes
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::SystemState;
    use bevy::prelude::{Entity, Query, World};

    use crate::cards;
    use crate::damage;
    use crate::effects;
    use crate::pt;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let attacker = cards::creature("Garruk's Companion", cards::Cost::default(), pt!(3, 2))
            .keyword(cards::trample());
        let source = world.spawn_empty().id();
        let defending = world.spawn_empty().id();
        let blocker = world
            .spawn((
                cards::creature("Grizzly Bears", cards::Cost::default(), pt!(2, 2)),
                cards::Blocking(source),
            ))
            .id();
        let mut blockers_state = SystemState::<
            Query<(
                Entity,
                &cards::CardDefinition,
                &cards::Blocking,
                Option<&damage::Damage>,
            )>,
        >::new(&mut world);
        let mut modifiers_state =
            SystemState::<Query<&effects::PowerToughnessModifier>>::new(&mut world);
        let mut needed = Vec::new();

        let blockers = blockers_state.get(&world).unwrap();
        let modifiers = modifiers_state.get(&world).unwrap();
        let writes = attacker.assigned_blocked_damage(
            source,
            3,
            defending,
            &blockers,
            &modifiers,
            &mut needed,
        );

        assert_eq!(
            writes,
            vec![
                damage::DamageDealt {
                    source: blocker,
                    target: source,
                    amount: 2,
                },
                damage::DamageDealt {
                    source,
                    target: blocker,
                    amount: 2,
                },
                damage::DamageDealt {
                    source,
                    target: defending,
                    amount: 1,
                },
            ]
        );
        assert_eq!(needed.len(), 0);
    }

    #[test]
    fn assigned_blocked_damage_counts_damage_already_marked_toward_lethal() {
        let mut world = World::new();
        let attacker = cards::creature("Garruk's Companion", cards::Cost::default(), pt!(3, 2))
            .keyword(cards::trample());
        let source = world.spawn_empty().id();
        let defending = world.spawn_empty().id();
        let blocker = world
            .spawn((
                cards::creature("Grizzly Bears", cards::Cost::default(), pt!(2, 2)),
                cards::Blocking(source),
                damage::Damage(1),
            ))
            .id();
        let mut blockers_state = SystemState::<
            Query<(
                Entity,
                &cards::CardDefinition,
                &cards::Blocking,
                Option<&damage::Damage>,
            )>,
        >::new(&mut world);
        let mut modifiers_state =
            SystemState::<Query<&effects::PowerToughnessModifier>>::new(&mut world);
        let mut needed = Vec::new();

        let blockers = blockers_state.get(&world).unwrap();
        let modifiers = modifiers_state.get(&world).unwrap();
        let writes = attacker.assigned_blocked_damage(
            source,
            3,
            defending,
            &blockers,
            &modifiers,
            &mut needed,
        );

        assert_eq!(
            writes,
            vec![
                damage::DamageDealt {
                    source: blocker,
                    target: source,
                    amount: 2,
                },
                damage::DamageDealt {
                    source,
                    target: blocker,
                    amount: 1,
                },
                damage::DamageDealt {
                    source,
                    target: defending,
                    amount: 2,
                },
            ]
        );
        assert_eq!(needed.len(), 0);
    }
}
