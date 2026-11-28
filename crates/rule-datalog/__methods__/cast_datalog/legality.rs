use bevy::prelude::Entity;

use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::CastDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn legality<G: frogmyr::traits::engine::Engine>(
    engine: &G,
    answers: &rules::Rules,
    spell: Entity,
    caster: players::PlayerId,
) -> Result<Option<rules::Legality>, TranslationError> {
    if !PlayDatalog::visible_entities(answers, caster).contains(&spell)
        || !answers.players_in_game(rules::Players::All, Some(caster)).contains(&caster)
    {
        return Ok(None);
    }
    let program = CastDatalog::cast_program()?;
    let facts = CastDatalog::snapshot(answers, caster);
    let derived = engine.evaluate(&program, &facts)?;
    let event = events::Cast { spell, caster };
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &PlayDatalog::VIOLATES,
            &[
                Value::Sym(CastDatalog::CAST),
                Value::Entity(spell.to_bits()),
                Value::Entity(caster.to_bits()),
                Value::Sym(*id),
            ],
        )
    };
    let mut reasons: Vec<rules::Why> = Vec::new();
    for restriction in events::Cast::restrictions() {
        if violated(restriction.id) {
            reasons.push(rules::Why::Restriction(restriction.id));
        }
    }
    for change in
        PlayDatalog::holder_changes(answers, &event, rules::Target::event::<events::Cast>())
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

    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use frogmyr::DatafrogEngine;
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::mana;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::DISPEL;
    use verdigris_rule_engine::testing::DIVINATION;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::LEYLINE_OF_ANTICIPATION;
    use verdigris_rule_engine::testing::LIGHTNING_BOLT;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::legality;
    use crate::CastDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        let bolt = app
            .world_mut()
            .spawn((
                (LIGHTNING_BOLT.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        app.world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality(&DatafrogEngine, &answers, bolt, first).unwrap(),
                    Some(answers.legality(&events::Cast {
                        spell: bolt,
                        caster: first,
                    }))
                );
            })
            .unwrap();
    }

    #[test]
    fn oracle_matches_when_casting_an_instant() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(bolt)]).unwrap();
        assert!(game.stack().has(LIGHTNING_BOLT));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE cast-instant candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_on_an_opponents_turn() {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2.active()]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(
                    &DatafrogEngine,
                    answers,
                    *divination,
                    players::PlayerId(*p2)
                ),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p2.pass_priority()]).unwrap();
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-opponent-turn candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_while_the_stack_is_not_empty() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DIVINATION]).mana_pool(mana![R, 2, U]),
            P2,
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT).unwrap();
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(bolt).targeting(p2)]).unwrap();
        assert!(game.stack().has(LIGHTNING_BOLT));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                assert_eq!(
                    CastDatalog::legality(
                        &DatafrogEngine,
                        answers,
                        *divination,
                        players::PlayerId(*p2)
                    ),
                    Ok(None)
                );
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-stack-nonempty candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_land_proposed_as_a_spell() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE cast-land candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_during_combat() {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::DeclareAttackers)
            .players([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(
                    &DatafrogEngine,
                    answers,
                    *divination,
                    players::PlayerId(*p2)
                ),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE cast-combat candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_on_an_opponents_turn_with_leyline() {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([DIVINATION]).mana_pool(mana![2, U]),
                P2.active(),
            ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(
                    &DatafrogEngine,
                    answers,
                    *divination,
                    players::PlayerId(*p2)
                ),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p2.pass_priority()]).unwrap();
        game.play([p1.cast(divination)]).unwrap();
        assert!(game.stack().has(DIVINATION));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-leyline-opponent-turn candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_during_combat_with_leyline() {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(1, turns::Step::DeclareAttackers).players([
                P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([DIVINATION]).mana_pool(mana![2, U]),
                P2,
            ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(
                    &DatafrogEngine,
                    answers,
                    *divination,
                    players::PlayerId(*p2)
                ),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(divination)]).unwrap();
        assert!(game.stack().has(DIVINATION));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-leyline-combat candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_sorcery_while_the_stack_is_not_empty_with_leyline() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([LEYLINE_OF_ANTICIPATION])
                .hand([LIGHTNING_BOLT, DIVINATION])
                .mana_pool(mana![R, 2, U]),
            P2.hand([DISPEL]).mana_pool(mana![U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT).unwrap();
        let [dispel] = game.player(p2).cards_in(HAND).exactly::<1>(DISPEL).unwrap();
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *dispel, players::PlayerId(*p1)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(bolt).targeting(p2).pass_priority()]).unwrap();
        let [bolt_on_stack] = game.stack().exactly::<1>(LIGHTNING_BOLT).unwrap();
        game.play([p2.cast(dispel).targeting(bolt_on_stack)]).unwrap();
        game.play([p2.pass_priority(), p1.cast(divination)]).unwrap();
        assert!(game.stack().has(DIVINATION));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-leyline-stack candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 4);
    }

    #[test]
    fn oracle_matches_for_an_opponents_sorcery_with_leyline() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([LEYLINE_OF_ANTICIPATION]),
            P2.hand([DIVINATION]).mana_pool(mana![2, U]),
        ]);
        let [divination] = game.player(p2).cards_in(HAND).exactly::<1>(DIVINATION).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(
                    &DatafrogEngine,
                    answers,
                    *divination,
                    players::PlayerId(*p1)
                ),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-leyline-opponent candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_land_with_leyline_of_anticipation() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                CastDatalog::legality(&DatafrogEngine, answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let started = Instant::now();
                    let datafrogged =
                        CastDatalog::legality(&DatafrogEngine, answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE cast-land-leyline candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }
}
