use bevy::prelude::{Entity, Query};

use std::collections::BTreeSet;

use crate::actions;
use crate::cards;
use crate::damage;
use crate::keywords;

pub fn assigned_blocked_damage(
    definition: &cards::CardDefinition,
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
    source_has_deathtouch: bool,
) -> Vec<damage::DamageDealt> {
    let mut writes = Vec::new();
    let mut blockers_of: Vec<(Entity, u32)> = Vec::new();
    for (blocker, blocking, marked, effective) in blockers.iter() {
        if **blocking != source {
            continue;
        }
        let lethal = effective
            .map(|power_toughness| **power_toughness)
            .map_or(0, |power_toughness| {
                u32::try_from(power_toughness.toughness).unwrap_or_default()
            })
            .saturating_sub(marked.map_or(0, |damage| **damage));
        let lethal = if source_has_deathtouch {
            lethal.min(1)
        } else {
            lethal
        };
        blockers_of.push((blocker, lethal));
        if !participants.contains(&blocker) {
            continue;
        }
        let Some(blocker_power_toughness) = effective.map(|power_toughness| **power_toughness)
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
    if amount == 0 || !participants.contains(&source) {
        return writes;
    }
    match blockers_of.as_slice() {
        [(blocker, lethal)] => {
            let into_blocker = if definition.has_keyword(keywords::StaticKeyword::Trample) {
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
        [] if definition.has_keyword(keywords::StaticKeyword::Trample) => {
            writes.push(damage::DamageDealt {
                source,
                target: defending,
                amount,
            });
        }
        [] => {}
        _ => {
            let mut legal: BTreeSet<Entity> =
                blockers_of.iter().map(|(blocker, _)| *blocker).collect();
            let trample_defender =
                definition.has_keyword(keywords::StaticKeyword::Trample).then_some(defending);
            if let Some(defender) = trample_defender {
                legal.insert(defender);
            }
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

    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::damage;
    use crate::mana;
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

        let attacker = (GARRUKS_COMPANION.definition)();
        let source = Entity::from(companion);
        let defending = Entity::from(p2);
        let bears_blocking = Entity::from(bears);
        let participants = BTreeSet::from([source, bears_blocking]);
        let mut needed = Vec::new();

        let writes = game.with_query::<(
            Entity,
            &cards::Blocking,
            Option<&damage::Damage>,
            Option<&cards::EffectivePT>,
        ), Vec<damage::DamageDealt>>(|blockers| {
            attacker.assigned_blocked_damage(
                source,
                3,
                defending,
                &blockers,
                &participants,
                &mut needed,
                false,
            )
        });

        assert_eq!(
            writes,
            vec![
                damage::DamageDealt {
                    source: bears_blocking,
                    target: source,
                    amount: 2,
                },
                damage::DamageDealt {
                    source,
                    target: bears_blocking,
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

        let attacker = (GARRUKS_COMPANION.definition)();
        let source = Entity::from(companion);
        let defending = Entity::from(p2);
        let spider_blocking = Entity::from(spider);
        let participants = BTreeSet::from([source, spider_blocking]);
        let mut needed = Vec::new();

        let writes = game.with_query::<(
            Entity,
            &cards::Blocking,
            Option<&damage::Damage>,
            Option<&cards::EffectivePT>,
        ), Vec<damage::DamageDealt>>(|blockers| {
            attacker.assigned_blocked_damage(
                source,
                3,
                defending,
                &blockers,
                &participants,
                &mut needed,
                true,
            )
        });

        assert_eq!(
            writes,
            vec![
                damage::DamageDealt {
                    source: spider_blocking,
                    target: source,
                    amount: 2,
                },
                damage::DamageDealt {
                    source,
                    target: spider_blocking,
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
        Ok(())
    }
}
