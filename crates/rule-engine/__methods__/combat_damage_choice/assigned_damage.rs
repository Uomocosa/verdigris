use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::Entity;

use crate::actions;
use crate::damage;
use crate::players;

pub fn assigned_damage(
    choice: &actions::CombatDamageChoice,
    player: players::PlayerId,
    assignments: &[actions::DamageAssignment],
) -> Option<Vec<damage::AssignedDamage>> {
    if player != choice.player || assignments.is_empty() {
        return None;
    }
    let mut seen: BTreeSet<Entity> = BTreeSet::new();
    let mut totals: BTreeMap<Entity, u32> = BTreeMap::new();
    let mut attributed: Vec<(Entity, Entity, u32)> = Vec::new();
    for assignment in assignments {
        let required =
            choice.needed.iter().find(|required| required.legal.contains(&assignment.target))?;
        if assignment.amount == 0 || !seen.insert(assignment.target) {
            return None;
        }
        attributed.push((required.source, assignment.target, assignment.amount));
        totals
            .entry(required.source)
            .and_modify(|total| *total = total.saturating_add(assignment.amount))
            .or_insert(assignment.amount);
    }
    let covers_all = totals.len() == choice.needed.len()
        && choice
            .needed
            .iter()
            .all(|required| totals.get(&required.source) == Some(&required.amount));
    if !covers_all {
        return None;
    }
    for required in &choice.needed {
        let Some(defender) = required.trample_defender else {
            continue;
        };
        if !assignments.iter().any(|assignment| assignment.target == defender) {
            continue;
        }
        let all_lethal = required.lethal_by_target.iter().all(|(blocker, lethal)| {
            assignments
                .iter()
                .find(|assignment| assignment.target == *blocker)
                .is_some_and(|assignment| assignment.amount >= *lethal)
        });
        if !all_lethal {
            return None;
        }
    }
    Some(
        attributed
            .into_iter()
            .map(|(source, target, amount)| damage::AssignedDamage {
                source,
                target,
                amount,
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::testing::*;

    use super::assigned_damage;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, p2, ..]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([GRIZZLY_BEARS, GIANT_SPIDER]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        let player = p1.id;
        let source = Entity::from(companion);
        let bears_blocking = Entity::from(bears);
        let spider_blocking = Entity::from(spider);
        let outsider = p2.id;

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [bears_blocking, spider_blocking].into_iter().collect(),
                trample_defender: None,
                lethal_by_target: BTreeMap::new(),
            }],
        };
        let split = [
            actions::DamageAssignment {
                target: bears_blocking,
                amount: 1,
            },
            actions::DamageAssignment {
                target: spider_blocking,
                amount: 1,
            },
        ];

        assert!(assigned_damage(&choice, player, &split).is_some());
        assert!(assigned_damage(&choice, outsider, &split).is_none());
        Ok(())
    }

    #[test]
    fn assigned_damage_rejects_short_changed_illegal_and_repeated_targets() -> TestResult {
        let (game, [p1, p2, ..]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([GRIZZLY_BEARS, GIANT_SPIDER]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        let player = p1.id;
        let source = Entity::from(companion);
        let bears_blocking = Entity::from(bears);
        let spider_blocking = Entity::from(spider);
        let outsider = p2.id;

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [bears_blocking, spider_blocking].into_iter().collect(),
                trample_defender: None,
                lethal_by_target: BTreeMap::new(),
            }],
        };

        let short = [actions::DamageAssignment {
            target: bears_blocking,
            amount: 1,
        }];
        assert!(assigned_damage(&choice, player, &short).is_none());

        let illegal = [
            actions::DamageAssignment {
                target: bears_blocking,
                amount: 1,
            },
            actions::DamageAssignment {
                target: *outsider,
                amount: 1,
            },
        ];
        assert!(assigned_damage(&choice, player, &illegal).is_none());

        let repeated = [
            actions::DamageAssignment {
                target: bears_blocking,
                amount: 1,
            },
            actions::DamageAssignment {
                target: bears_blocking,
                amount: 1,
            },
        ];
        assert!(assigned_damage(&choice, player, &repeated).is_none());

        let zero = [actions::DamageAssignment {
            target: bears_blocking,
            amount: 0,
        }];
        assert!(assigned_damage(&choice, player, &zero).is_none());
        Ok(())
    }
}
