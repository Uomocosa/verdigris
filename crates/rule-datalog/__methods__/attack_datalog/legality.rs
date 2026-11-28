use bevy::prelude::Entity;

use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::AttackDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn legality<G: frogmyr::traits::engine::Engine>(
    engine: &G,
    answers: &rules::Rules,
    attacker: Entity,
    viewer: players::PlayerId,
) -> Result<Option<rules::Legality>, TranslationError> {
    if !PlayDatalog::visible_entities(answers, viewer).contains(&attacker)
        || !answers.players_in_game(rules::Players::All, Some(viewer)).contains(&viewer)
    {
        return Ok(None);
    }
    let program = AttackDatalog::attack_program()?;
    let facts = AttackDatalog::snapshot(answers, viewer);
    let derived = engine.evaluate(&program, &facts)?;
    let event = events::Attack { attacker };
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &PlayDatalog::VIOLATES,
            &[
                Value::Sym(AttackDatalog::ATTACK),
                Value::Entity(attacker.to_bits()),
                Value::Entity(viewer.to_bits()),
                Value::Sym(*id),
            ],
        )
    };
    let mut reasons: Vec<rules::Why> = Vec::new();
    for restriction in events::Attack::restrictions() {
        if violated(restriction.id) {
            reasons.push(rules::Why::Restriction(restriction.id));
        }
    }
    for change in
        PlayDatalog::holder_changes(answers, &event, rules::Target::event::<events::Attack>())
    {
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

    use frogmyr::DatafrogEngine;
    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::mana;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::MASTER_OF_THE_FEAST;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;
    use verdigris_rule_engine::testing::RAGING_GOBLIN;
    use verdigris_rule_engine::testing::SPHINX_OF_THE_SECOND_SUN;
    use verdigris_rule_engine::testing::TEST_BATTLE_FERVOR;
    use verdigris_rule_engine::testing::TEST_CALL_TO_ARMS;
    use verdigris_rule_engine::testing::TEST_EAGER_WALL;
    use verdigris_rule_engine::testing::TEST_HASTY_BANNER;
    use verdigris_rule_engine::testing::TEST_RESTLESS_WATCHER;
    use verdigris_rule_engine::testing::TEST_SILENT_ARBITER;
    use verdigris_rule_engine::testing::TEST_WALL_OF_DREAD;
    use verdigris_rule_engine::testing::TEST_WAR_DRUMS;
    use verdigris_rule_engine::testing::TUNDRA;
    use verdigris_rule_engine::testing::WALKING_CORPSE;
    use verdigris_rule_engine::testing::WALL_OF_STONE;
    use verdigris_rule_engine::turns;

    use super::legality;
    use crate::AttackDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([FOREST]).battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.pass_priority();
        let viewer = players::PlayerId(*p1);
        let other = players::PlayerId(*p2);
        game.read_rules(|answers, _legal| {
            assert_eq!(
                legality(&DatafrogEngine, answers, *bears, viewer).unwrap(),
                Some(answers.legality(&events::Attack { attacker: *bears }))
            );
            assert_eq!(
                legality(&DatafrogEngine, answers, *bears, other).unwrap(),
                Some(answers.legality(&events::Attack { attacker: *bears }))
            );
            assert_eq!(
                legality(&DatafrogEngine, answers, *forest, viewer).unwrap(),
                Some(answers.legality(&events::Attack { attacker: *forest }))
            );
            assert_eq!(legality(&DatafrogEngine, answers, *forest, other), Ok(None));
        })
        .unwrap();
    }

    #[test]
    fn oracle_matches_for_a_creature_controlled_by_the_opponent() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-opponent-creature candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_noncreature_permanent() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-noncreature candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_tapped_creature() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE attack-tapped candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_creature_that_entered_this_turn() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([WALKING_CORPSE]).mana_pool(mana![1, B]), P2]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE).unwrap();
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()]).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-summoning-sick candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_creature_with_defender() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([WALL_OF_STONE]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE attack-defender candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_haste_waiving_only_its_own_creature() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([RAGING_GOBLIN, WALKING_CORPSE]).mana_pool(mana![1, R, B]),
            P2,
        ]);
        let [goblin] = game.player(p1).cards_in(HAND).exactly::<1>(RAGING_GOBLIN).unwrap();
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE).unwrap();
        game.play([p1.cast(goblin).pass_priority(), p2.pass_priority()]).unwrap();
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()]).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE attack-haste candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_creature_that_must_attack() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([MASTER_OF_THE_FEAST, GRIZZLY_BEARS]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-must-attack candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_static_haste_grant() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_HASTY_BANNER]).hand([WALKING_CORPSE]).mana_pool(mana![1, B]),
            P2,
        ]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE).unwrap();
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()]).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-hasty-banner candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_static_defender_loss() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WALL_OF_STONE]),
            P2.battlefield([TEST_WALL_OF_DREAD]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-defender-loss candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_attack_as_though_it_had_haste() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_BATTLE_FERVOR]).hand([WALKING_CORPSE]).mana_pool(mana![1, B]),
            P2,
        ]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE).unwrap();
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()]).unwrap();
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-as-though-haste candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_static_must_attack_grant() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([TEST_WAR_DRUMS, GRIZZLY_BEARS]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-war-drums candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_static_must_attack_rule_answer() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([TEST_CALL_TO_ARMS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-call-to-arms candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_creature_that_waives_untapped() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield_tapped([TEST_RESTLESS_WATCHER]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-restless-watcher candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_creature_that_waives_defender() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TEST_EAGER_WALL]), P2]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-eager-wall candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_when_a_declaration_cap_leaves_each_attack_legal() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, GRIZZLY_BEARS]),
            P2.battlefield([TEST_SILENT_ARBITER]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let viewer = players::PlayerId(*p1);
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-silent-arbiter candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 3);
    }

    #[test]
    fn oracle_matches_as_summoning_sickness_lifts() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([WALKING_CORPSE])
                .mana_pool(mana![1, B])
                .battlefield([SPHINX_OF_THE_SECOND_SUN]),
            P2,
        ]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE).unwrap();
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()]).unwrap();
        let viewer = players::PlayerId(*p1);
        let (sick, sick_datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-sickness-sick candidates={sick} datafrog_time={sick_datafrog_time:?}"
        );
        assert_eq!(sick, 2);
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();
        game.play([p2.end_turn(), p1.end_turn()]).unwrap();
        game.play([p1.pass_to(turns::Step::Upkeep)]).unwrap();
        let (lifted, lifted_datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, viewer) {
                    let expected = answers.legality(&events::Attack { attacker: card });
                    let started = Instant::now();
                    let datafrogged =
                        AttackDatalog::legality(&DatafrogEngine, answers, card, viewer).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE attack-sickness-lifted candidates={lifted} datafrog_time={lifted_datafrog_time:?}"
        );
        assert_eq!(lifted, 2);
    }
}
