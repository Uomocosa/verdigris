use bevy::prelude::Entity;

use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::BlockDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn legality(
    answers: &rules::Rules,
    blocker: Entity,
    attacker: Entity,
    viewer: players::PlayerId,
) -> Result<Option<rules::Legality>, TranslationError> {
    let visible = PlayDatalog::visible_entities(answers, viewer);
    if !visible.contains(&blocker) || !visible.contains(&attacker) {
        return Ok(None);
    }
    let program = BlockDatalog::block_program()?;
    let facts = BlockDatalog::snapshot(answers, viewer);
    let derived = program.evaluate(&facts)?;
    let event = events::DeclareBlocker { blocker, attacker };
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &PlayDatalog::VIOLATES,
            &[
                Value::Sym(BlockDatalog::BLOCK),
                Value::Entity(blocker.to_bits()),
                Value::Entity(attacker.to_bits()),
                Value::Sym(*id),
            ],
        )
    };
    let mut reasons: Vec<rules::Why> = Vec::new();
    for restriction in events::DeclareBlocker::restrictions() {
        if violated(restriction.id) {
            reasons.push(rules::Why::Restriction(restriction.id));
        }
    }
    let target = rules::Target::event::<events::DeclareBlocker>();
    for change in PlayDatalog::holder_changes(answers, &event, target) {
        let rules::Change::Restrict { restriction, .. } = change else {
            continue;
        };
        if violated(restriction.id) {
            reasons.push(rules::Why::Restriction(restriction.id));
        }
    }
    if !answers.performs(&event) {
        reasons.push(rules::Why::NotPerformable);
    }
    if reasons.is_empty() {
        Ok(Some(rules::Legality::Legal))
    } else {
        Ok(Some(rules::Legality::Illegal(reasons)))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::time::Instant;

    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::IRON_GOLEM;
    use verdigris_rule_engine::testing::JUMP;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;
    use verdigris_rule_engine::testing::TEST_FLYING_REACH;
    use verdigris_rule_engine::testing::TEST_GROUNDING_MIST;
    use verdigris_rule_engine::testing::TEST_LOW_SKIMMER;
    use verdigris_rule_engine::testing::TEST_LURE_OF_BATTLE;
    use verdigris_rule_engine::testing::TEST_SKY_COURT;
    use verdigris_rule_engine::testing::TEST_TAPPED_SENTRY;
    use verdigris_rule_engine::testing::TEST_TAUNTING_FIELD;
    use verdigris_rule_engine::testing::TEST_TIMID_FIELD;
    use verdigris_rule_engine::testing::TUNDRA;
    use verdigris_rule_engine::testing::TYPHOID_RATS;
    use verdigris_rule_engine::testing::WATCHWOLF;
    use verdigris_rule_engine::testing::WIND_DRAKE;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::legality;
    use crate::BlockDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([FOREST]).battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.pass_priority();
        let viewer = players::PlayerId(*p1);
        let other = players::PlayerId(*p2);
        game.read_rules(|answers, _legal| {
            assert_eq!(
                legality(answers, *bears_1, *bears_2, viewer).unwrap(),
                Some(answers.legality(&events::DeclareBlocker {
                    blocker: *bears_1,
                    attacker: *bears_2,
                }))
            );
            assert_eq!(
                legality(answers, *bears_2, *bears_1, viewer).unwrap(),
                Some(answers.legality(&events::DeclareBlocker {
                    blocker: *bears_2,
                    attacker: *bears_1,
                }))
            );
            assert_eq!(
                legality(answers, *bears_1, *bears_2, other).unwrap(),
                Some(answers.legality(&events::DeclareBlocker {
                    blocker: *bears_1,
                    attacker: *bears_2,
                }))
            );
            assert_eq!(
                legality(answers, *forest, *bears_2, viewer).unwrap(),
                Some(answers.legality(&events::DeclareBlocker {
                    blocker: *forest,
                    attacker: *bears_2,
                }))
            );
            assert_eq!(legality(answers, *forest, *bears_2, other), Ok(None));
            assert_eq!(legality(answers, *bears_2, *forest, other), Ok(None));
        })
        .unwrap();
    }

    #[test]
    fn oracle_matches_for_a_tapped_blocker() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-tapped-blocker candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_a_tapped_sentry_waiving_untapped() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield_tapped([TEST_TAPPED_SENTRY]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-tapped-sentry candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_a_non_attacking_attacker() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-non-attacking candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }

    #[test]
    fn oracle_matches_for_flying_evasion() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WIND_DRAKE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([drake]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-flying candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_flying_reach_evasion() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_FLYING_REACH]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [skimmer] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_FLYING_REACH).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([skimmer]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-flying-reach candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_jump_granting_flying() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([JUMP]).battlefield([GRIZZLY_BEARS, TUNDRA]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [jump] = game.player(p1).cards_in(HAND).exactly::<1>(JUMP).unwrap();
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::Blue)))
                .cast(jump)
                .targeting(bears_1)
                .pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-jump candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }

    #[test]
    fn oracle_matches_for_a_creature_that_must_block() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([IRON_GOLEM]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-must-block candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_a_static_must_block_grant() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, TEST_TAUNTING_FIELD]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-taunting-field candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }

    #[test]
    fn oracle_matches_for_a_static_must_block_rule_answer() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, TEST_LURE_OF_BATTLE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-lure-of-battle candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }

    #[test]
    fn oracle_matches_for_flying_loss() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WIND_DRAKE]),
            P2.battlefield([GRIZZLY_BEARS, TEST_GROUNDING_MIST]),
        ]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([drake]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-grounding-mist candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }

    #[test]
    fn oracle_matches_for_timid_field() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS, WATCHWOLF, TEST_TIMID_FIELD]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-timid-field candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 4);
    }

    #[test]
    fn oracle_matches_for_sky_court() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([WIND_DRAKE, GRIZZLY_BEARS, TEST_SKY_COURT]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-sky-court candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 4);
    }

    #[test]
    fn oracle_matches_for_low_skimmer_waiving_flying() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_LOW_SKIMMER]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [skimmer] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_LOW_SKIMMER).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([skimmer]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-low-skimmer candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn oracle_matches_for_a_blocker_controlled_by_the_attacker() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p2);
        let (pairs, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let visible = PlayDatalog::visible_entities(answers, viewer);
                let mut blockers = Vec::new();
                let mut attackers = Vec::new();
                for card in &visible {
                    if let Ok(found) = answers.entities.get(*card) {
                        if found.get::<cards::Attacking>().is_some() {
                            attackers.push(*card);
                        }
                        if let Some(in_zone) = found.get::<zones::InZone>()
                            && let Ok((_, zone, _)) = answers.zones.get(**in_zone)
                            && matches!(*zone, zones::Zone::Battlefield)
                        {
                            blockers.push(*card);
                        }
                    }
                }
                blockers.sort();
                attackers.sort();
                let mut pairs = 0;
                let mut datafrog_time = Duration::ZERO;
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            BlockDatalog::legality(answers, *blocker, *attacker, viewer).unwrap();
                        datafrog_time += started.elapsed();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                (pairs, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE block-not-yours candidates={pairs} datafrog_time={datafrog_time:?}");
        assert_eq!(pairs, 3);
    }
}
