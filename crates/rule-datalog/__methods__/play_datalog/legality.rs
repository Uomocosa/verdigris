use bevy::prelude::Entity;

use verdigris_datalog_ir::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn legality<G: verdigris_datalog_ir::traits::engine::Engine>(
    engine: &G,
    answers: &rules::Rules,
    card: Entity,
    player: players::PlayerId,
) -> Result<rules::Legality, TranslationError> {
    let program = PlayDatalog::play_program()?;
    let facts = PlayDatalog::snapshot(answers, player);
    let derived = engine.evaluate(&program, &facts)?;
    let event = events::Play { card, player };
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &PlayDatalog::VIOLATES,
            &[
                Value::Sym(PlayDatalog::PLAY),
                Value::Entity(card.to_bits()),
                Value::Entity(player.to_bits()),
                Value::Sym(*id),
            ],
        )
    };
    let mut reasons: Vec<rules::Why> = Vec::new();
    for restriction in events::Play::restrictions() {
        if violated(restriction.id) {
            reasons.push(rules::Why::Restriction(restriction.id));
        }
    }
    for change in
        PlayDatalog::holder_changes(answers, &event, rules::Target::event::<events::Play>())
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
        Ok(rules::Legality::Legal)
    } else {
        Ok(rules::Legality::Illegal(reasons))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::time::Instant;

    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use verdigris_datalog_datafrog::DatafrogEngine;
    use verdigris_datalog_nemo::NemoEngine;
    use verdigris_rule_engine::GRAVEYARD;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::effects;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::legality;
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
        let forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let bears = app
            .world_mut()
            .spawn((
                (GRIZZLY_BEARS.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        app.world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality(&DatafrogEngine, &answers, forest, first).unwrap(),
                    answers.legality(&events::Play {
                        card: forest,
                        player: first,
                    })
                );
                assert_eq!(
                    legality(&NemoEngine, &answers, bears, first).unwrap(),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(
                        events::play::LAND_CARD
                    )])
                );
            })
            .unwrap();
    }

    #[test]
    fn oracle_matches_on_a_played_land() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut()
                .spawn((
                    players::Player,
                    players::LandsPlayedThisTurn {
                        count: 1,
                        turn_number: 1,
                    },
                ))
                .id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let battlefield =
            app.world_mut().spawn((zones::Zone::Battlefield, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(battlefield),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE played-land candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_on_a_second_land_in_the_same_turn() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut()
                .spawn((
                    players::Player,
                    players::LandsPlayedThisTurn {
                        count: 1,
                        turn_number: 1,
                    },
                ))
                .id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
        let battlefield =
            app.world_mut().spawn((zones::Zone::Battlefield, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        let _played = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(battlefield),
                cards::Owner(first),
            ))
            .id();
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let _bears = app
            .world_mut()
            .spawn((
                (GRIZZLY_BEARS.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE second-land candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 4);
    }
    #[test]
    fn oracle_matches_on_azusa_allowing_two_additional_lands() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut()
                .spawn((
                    players::Player,
                    players::LandsPlayedThisTurn {
                        count: 3,
                        turn_number: 1,
                    },
                ))
                .id(),
        );
        let second = players::PlayerId(
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
        app.world_mut().spawn(effects::ContinuousEffect {
            affected: *first,
            source: None,
            modification: effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::value::<rules::LandPlays>(),
                change: rules::Change::Increase(2),
            }),
            until: effects::Duration::WhileSourceOnBattlefield,
            timestamp: 0,
        });
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE azusa candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_on_a_land_during_an_opponents_turn() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 2,
            active: second,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(second);
        app.insert_resource(state);
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE opponent-turn candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_on_playing_a_land_from_the_graveyard() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let graveyard = app.world_mut().spawn((zones::Zone::Graveyard, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        app.world_mut().spawn(effects::ContinuousEffect {
            affected: *first,
            source: None,
            modification: effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::play::FROM_HAND),
                change: rules::Change::Allow {
                    holder: rules::ArgRef::Arg(events::Play::PLAYER),
                    when: rules::in_zone(GRAVEYARD, events::Play::PLAYER).of(events::Play::CARD),
                },
            }),
            until: effects::Duration::WhileSourceOnBattlefield,
            timestamp: 0,
        });
        let _buried = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(graveyard),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE crucible-graveyard candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_on_a_land_in_the_graveyard_without_crucible() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let graveyard = app.world_mut().spawn((zones::Zone::Graveyard, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(graveyard),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE graveyard-without-crucible candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_on_an_opponents_graveyard_with_crucible() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let other_graveyard =
            app.world_mut().spawn((zones::Zone::Graveyard, cards::Owner(second))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        app.world_mut().spawn(effects::ContinuousEffect {
            affected: *first,
            source: None,
            modification: effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::play::FROM_HAND),
                change: rules::Change::Allow {
                    holder: rules::ArgRef::Arg(events::Play::PLAYER),
                    when: rules::in_zone(GRAVEYARD, events::Play::PLAYER).of(events::Play::CARD),
                },
            }),
            until: effects::Duration::WhileSourceOnBattlefield,
            timestamp: 0,
        });
        let _forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(other_graveyard),
                cards::Owner(second),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE opponent-graveyard candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_on_crucible_during_an_opponents_turn() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let graveyard = app.world_mut().spawn((zones::Zone::Graveyard, cards::Owner(first))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 2,
            active: second,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(second);
        app.insert_resource(state);
        app.world_mut().spawn(effects::ContinuousEffect {
            affected: *first,
            source: None,
            modification: effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::play::FROM_HAND),
                change: rules::Change::Allow {
                    holder: rules::ArgRef::Arg(events::Play::PLAYER),
                    when: rules::in_zone(GRAVEYARD, events::Play::PLAYER).of(events::Play::CARD),
                },
            }),
            until: effects::Duration::WhileSourceOnBattlefield,
            timestamp: 0,
        });
        let _buried = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(graveyard),
                cards::Owner(first),
            ))
            .id();
        let mut datafrog_times = Vec::new();
        let mut nemo_times = Vec::new();
        for viewer in [first, second] {
            let (datafrog_spent, nemo_spent) = app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    let mut datafrog_spent = Vec::new();
                    let mut nemo_spent = Vec::new();
                    for card in PlayDatalog::visible_entities(&answers, viewer) {
                        let expected = answers.legality(&events::Play {
                            card,
                            player: viewer,
                        });
                        let started = Instant::now();
                        let datafrogged =
                            PlayDatalog::legality(&DatafrogEngine, &answers, card, viewer).unwrap();
                        datafrog_spent.push(started.elapsed());
                        let started = Instant::now();
                        let nemoed =
                            PlayDatalog::legality(&NemoEngine, &answers, card, viewer).unwrap();
                        nemo_spent.push(started.elapsed());
                        assert_eq!(datafrogged, expected);
                        assert_eq!(nemoed, expected);
                    }
                    (datafrog_spent, nemo_spent)
                })
                .unwrap();
            datafrog_times.extend(datafrog_spent);
            nemo_times.extend(nemo_spent);
        }
        let candidates = datafrog_times.len();
        let datafrog_time: Duration = datafrog_times.iter().copied().sum();
        let nemo_time: Duration = nemo_times.iter().copied().sum();
        eprintln!(
            "ORACLE crucible-opponent-turn candidates={candidates} datafrog_time={datafrog_time:?} nemo_time={nemo_time:?}"
        );
        assert_eq!(candidates, 2);
    }
}
