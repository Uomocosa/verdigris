use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::Entity;

use crate::actions;
use crate::damage;

pub fn assigned_damage(
    choice: &actions::CombatDamageChoice,
    player: Entity,
    assignments: &[actions::DamageAssignment],
) -> Option<Vec<damage::DamageDealt>> {
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
    Some(
        attributed
            .into_iter()
            .map(|(source, target, amount)| damage::DamageDealt {
                source,
                target,
                amount,
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
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
        let player = Entity::from(p1);
        let source = Entity::from(companion);
        let first = Entity::from(bears);
        let second = Entity::from(spider);
        let outsider = Entity::from(p2);

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [first, second].into_iter().collect(),
            }],
        };
        let split = [
            actions::DamageAssignment {
                target: first,
                amount: 1,
            },
            actions::DamageAssignment {
                target: second,
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
        let player = Entity::from(p1);
        let source = Entity::from(companion);
        let first = Entity::from(bears);
        let second = Entity::from(spider);
        let outsider = Entity::from(p2);

        let choice = actions::CombatDamageChoice {
            player,
            needed: vec![actions::RequiredCombatDamage {
                source,
                amount: 2,
                legal: [first, second].into_iter().collect(),
            }],
        };

        let short = [actions::DamageAssignment {
            target: first,
            amount: 1,
        }];
        assert!(assigned_damage(&choice, player, &short).is_none());

        let illegal = [
            actions::DamageAssignment {
                target: first,
                amount: 1,
            },
            actions::DamageAssignment {
                target: outsider,
                amount: 1,
            },
        ];
        assert!(assigned_damage(&choice, player, &illegal).is_none());

        let repeated = [
            actions::DamageAssignment {
                target: first,
                amount: 1,
            },
            actions::DamageAssignment {
                target: first,
                amount: 1,
            },
        ];
        assert!(assigned_damage(&choice, player, &repeated).is_none());

        let zero = [actions::DamageAssignment {
            target: first,
            amount: 0,
        }];
        assert!(assigned_damage(&choice, player, &zero).is_none());
        Ok(())
    }
}
