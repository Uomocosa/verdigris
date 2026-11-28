use bevy::prelude::{Entity, Query};

use std::collections::{BTreeMap, BTreeSet};

use crate::actions;
use crate::cards;
use crate::damage;
use crate::events;
use crate::rules;

pub fn assigned_blocked_damage(
    answers: rules::Rules,
    source: Entity,
    amount: u32,
    defending: Entity,
    blockers: &Query<(
        Entity,
        &cards::Blocking,
        Option<&damage::Damage>,
        Option<&cards::EffectivePT>,
    )>,
    participants: &BTreeSet<Entity>,
    needed: &mut Vec<actions::RequiredCombatDamage>,
    blocker_needed: &mut Vec<actions::RequiredCombatDamage>,
) -> Vec<damage::AssignedDamage> {
    let mut writes = Vec::new();
    let mut blockers_of: Vec<(Entity, u32)> = Vec::new();
    for (blocker, blocking, marked, effective) in blockers.iter() {
        let attackers = blocking.attackers();
        if !attackers.contains(&source) {
            continue;
        }
        let remaining = effective
            .map_or(0, |pt| u32::try_from(pt.toughness).unwrap_or_default())
            .saturating_sub(marked.map_or(0, |damage| **damage));
        let lethal = (1..=remaining)
            .find(|amount| {
                answers.defines(
                    &rules::Definition::lethal_damage(),
                    &events::AssignCombatDamage {
                        target: blocker,
                        source,
                        amount: *amount,
                    },
                )
            })
            .unwrap_or(1);
        blockers_of.push((blocker, lethal));
        if !participants.contains(&blocker) {
            continue;
        }
        let Some(blocker_amount) =
            effective.and_then(|pt| u32::try_from(pt.power).ok()).filter(|amount| *amount > 0)
        else {
            continue;
        };
        if attackers.len() > 1 && !blocker_needed.iter().any(|needed| needed.source == blocker) {
            blocker_needed.push(actions::RequiredCombatDamage {
                source: blocker,
                amount: blocker_amount,
                legal: attackers.into_iter().collect(),
                trample_defender: None,
                lethal_by_target: BTreeMap::new(),
            });
            continue;
        }
        writes.push(damage::AssignedDamage {
            source: blocker,
            target: source,
            amount: blocker_amount,
        });
    }
    if amount == 0 || !participants.contains(&source) {
        return writes;
    }
    let may_assign_to_defending = answers.legal(&events::AssignCombatDamage {
        target: defending,
        source,
        amount,
    });
    match blockers_of.as_slice() {
        [(blocker, lethal)] => {
            let into_blocker = if may_assign_to_defending {
                amount.min(*lethal)
            } else {
                amount
            };
            let into_defender = amount.saturating_sub(into_blocker);
            writes.extend(
                [
                    (source, *blocker, into_blocker),
                    (source, defending, into_defender),
                ]
                .into_iter()
                .filter(|(_, _, amount)| *amount > 0)
                .map(|(source, target, amount)| damage::AssignedDamage {
                    source,
                    target,
                    amount,
                }),
            );
        }
        [] if may_assign_to_defending => {
            writes.push(damage::AssignedDamage {
                source,
                target: defending,
                amount,
            });
        }
        [] => {}
        _ => {
            let mut legal: BTreeSet<Entity> =
                blockers_of.iter().map(|(blocker, _)| *blocker).collect();
            let trample_defender = may_assign_to_defending.then_some(defending);
            legal.extend(trample_defender);
            needed.push(actions::RequiredCombatDamage {
                source,
                amount,
                legal,
                trample_defender,
                lethal_by_target: blockers_of.iter().copied().collect(),
            });
        }
    }
    writes
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Entity, Query};

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::damage;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([companion]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(bears, companion)]),
        ])?;

        let source = Entity::from(companion);
        let defending = Entity::from(p2);
        let bears_blocking = Entity::from(bears);
        let participants = BTreeSet::from([source, bears_blocking]);

        let (writes, needed, blocker_needed) = game
            .app
            .world_mut()
            .run_system_once(
                move |blockers: Query<(
                    Entity,
                    &cards::Blocking,
                    Option<&damage::Damage>,
                    Option<&cards::EffectivePT>,
                )>,
                      answers: rules::Rules| {
                    let mut needed = Vec::new();
                    let mut blocker_needed = Vec::new();
                    let writes = cards::CardDefinition::assigned_blocked_damage(
                        answers,
                        source,
                        3,
                        defending,
                        &blockers,
                        &participants,
                        &mut needed,
                        &mut blocker_needed,
                    );
                    (writes, needed, blocker_needed)
                },
            )
            .unwrap();

        assert_eq!(
            writes,
            vec![
                damage::AssignedDamage {
                    source: bears_blocking,
                    target: source,
                    amount: 2,
                },
                damage::AssignedDamage {
                    source,
                    target: bears_blocking,
                    amount: 2,
                },
                damage::AssignedDamage {
                    source,
                    target: defending,
                    amount: 1,
                },
            ]
        );
        assert_eq!(needed.len(), 0);
        assert_eq!(blocker_needed.len(), 0);
        Ok(())
    }

    #[test]
    fn assigned_blocked_damage_counts_damage_already_marked_toward_lethal() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]).hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GIANT_SPIDER]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        game.play([
            p1.cast(bolt).targeting(spider).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([companion]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(spider, companion)]),
        ])?;

        let source = Entity::from(companion);
        let defending = Entity::from(p2);
        let spider_blocking = Entity::from(spider);
        let participants = BTreeSet::from([source, spider_blocking]);

        let (writes, needed, blocker_needed) = game
            .app
            .world_mut()
            .run_system_once(
                move |blockers: Query<(
                    Entity,
                    &cards::Blocking,
                    Option<&damage::Damage>,
                    Option<&cards::EffectivePT>,
                )>,
                      answers: rules::Rules| {
                    let mut needed = Vec::new();
                    let mut blocker_needed = Vec::new();
                    let writes = cards::CardDefinition::assigned_blocked_damage(
                        answers,
                        source,
                        3,
                        defending,
                        &blockers,
                        &participants,
                        &mut needed,
                        &mut blocker_needed,
                    );
                    (writes, needed, blocker_needed)
                },
            )
            .unwrap();

        assert_eq!(
            writes,
            vec![
                damage::AssignedDamage {
                    source: spider_blocking,
                    target: source,
                    amount: 2,
                },
                damage::AssignedDamage {
                    source,
                    target: spider_blocking,
                    amount: 1,
                },
                damage::AssignedDamage {
                    source,
                    target: defending,
                    amount: 2,
                },
            ]
        );
        assert_eq!(needed.len(), 0);
        assert_eq!(blocker_needed.len(), 0);
        Ok(())
    }
}
