use bevy::prelude::Entity;

use crate::events;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Value;

use crate::legality;

pub fn legality(
    answers: &rules::Rules,
    blocker: Entity,
    attacker: Entity,
    viewer: players::PlayerId,
) -> Result<Option<rules::Legality>, legality::TranslationError> {
    let visible = legality::PlayDatalog::visible_entities(answers, viewer);
    if !visible.contains(&blocker) || !visible.contains(&attacker) {
        return Ok(None);
    }
    let event = events::DeclareBlocker { blocker, attacker };
    let program = legality::BlockDatalog::block_program(answers, &event)?;
    let facts = legality::BlockDatalog::snapshot(answers, viewer);
    let derived = program.evaluate(&facts)?;
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &legality::PlayDatalog::VIOLATES,
            &[
                Value::Sym(legality::BlockDatalog::BLOCK),
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
    for change in legality::PlayDatalog::holder_changes(answers, &event, target) {
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

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::events;
    use crate::players;
    use crate::testing::FOREST;
    use crate::testing::GRIZZLY_BEARS;
    use crate::testing::IRON_GOLEM;
    use crate::testing::JUMP;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;
    use crate::testing::TEST_FLYING_REACH;
    use crate::testing::TEST_GROUNDING_MIST;
    use crate::testing::TEST_LOW_SKIMMER;
    use crate::testing::TEST_LURE_OF_BATTLE;
    use crate::testing::TEST_SKY_COURT;
    use crate::testing::TEST_TAPPED_SENTRY;
    use crate::testing::TEST_TAUNTING_FIELD;
    use crate::testing::TEST_TIMID_FIELD;
    use crate::testing::TUNDRA;
    use crate::testing::TYPHOID_RATS;
    use crate::testing::WATCHWOLF;
    use crate::testing::WIND_DRAKE;
    use crate::turns;
    use crate::zones;

    use super::legality;

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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-tapped-blocker candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-tapped-sentry candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-non-attacking candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-flying candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-flying-reach candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-jump candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-must-block candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-taunting-field candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-lure-of-battle candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-grounding-mist candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-timid-field candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-sky-court candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-low-skimmer candidates={pairs}");
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
        let pairs = game
            .read_rules(|answers, _legal| {
                let visible = legality::PlayDatalog::visible_entities(answers, viewer);
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
                for blocker in &blockers {
                    for attacker in &attackers {
                        let expected = answers.legality(&events::DeclareBlocker {
                            blocker: *blocker,
                            attacker: *attacker,
                        });
                        let datafrogged =
                            legality::BlockDatalog::legality(answers, *blocker, *attacker, viewer)
                                .unwrap();
                        assert_eq!(datafrogged, Some(expected));
                        pairs += 1;
                    }
                }
                pairs
            })
            .unwrap();
        eprintln!("ORACLE block-not-yours candidates={pairs}");
        assert_eq!(pairs, 3);
    }
}
