use bevy::prelude::Entity;

use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn legality(
    answers: &rules::Rules,
    card: Entity,
    player: players::PlayerId,
) -> Result<Option<rules::Legality>, TranslationError> {
    if !PlayDatalog::visible_entities(answers, player).contains(&card)
        || !answers.players_in_game(rules::Players::All, Some(player)).contains(&player)
    {
        return Ok(None);
    }
    let event = events::Play { card, player };
    let program = PlayDatalog::play_program(answers, &event)?;
    let facts = PlayDatalog::snapshot(answers, player);
    let derived = program.evaluate(&facts)?;
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
    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::GRAVEYARD;
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::mana;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::AZUSA_LOST_BUT_SEEKING;
    use verdigris_rule_engine::testing::CRUCIBLE_OF_WORLDS;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::MIND_ROT;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;
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
                    legality(&answers, forest, first).unwrap(),
                    Some(answers.legality(&events::Play {
                        card: forest,
                        player: first,
                    }))
                );
                assert_eq!(
                    legality(&answers, bears, first).unwrap(),
                    Some(rules::Legality::Illegal(vec![rules::Why::Restriction(
                        events::play::LAND_CARD
                    )]))
                );
            })
            .unwrap();
    }

    #[test]
    fn oracle_matches_on_a_played_land() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest)]).unwrap();
        assert!(game.moved(forest).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE played-land candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_on_a_second_land_in_the_same_turn() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([FOREST, FOREST, GRIZZLY_BEARS]), P2]);
        let [forest_1, forest_2] = game.player(p1).cards_in(HAND).exactly::<2>(FOREST).unwrap();
        let [_bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest_1, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest_1)]).unwrap();
        assert!(game.moved(forest_1).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                let hidden = (*forest_2, players::PlayerId(*p2));
                assert_eq!(PlayDatalog::legality(answers, hidden.0, hidden.1), Ok(None));
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE second-land candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 3);
    }
    #[test]
    fn oracle_matches_on_azusa_allowing_two_additional_lands() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([AZUSA_LOST_BUT_SEEKING]).hand([FOREST, FOREST, FOREST, FOREST]),
            P2,
        ]);
        let [forest_1, forest_2, forest_3, forest_4] =
            game.player(p1).cards_in(HAND).exactly::<4>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest_1, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest_1)]).unwrap();
        game.play([p1.play_land(forest_2)]).unwrap();
        game.play([p1.play_land(forest_3)]).unwrap();
        assert!(game.moved(forest_3).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                assert_eq!(
                    PlayDatalog::legality(answers, *forest_4, players::PlayerId(*p2)),
                    Ok(None)
                );
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE azusa candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 5);
    }

    #[test]
    fn oracle_matches_on_a_land_during_an_opponents_turn() {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([FOREST]), P2.active()]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
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
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!("ORACLE opponent-turn candidates={candidates} datafrog_time={datafrog_time:?}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_on_playing_a_land_from_the_graveyard() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]),
            P2,
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT).unwrap();
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *mind_rot, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));
        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST).unwrap();
        let [mind_rot_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(MIND_ROT).unwrap();
        game.play([p1.arrange_graveyard([forest_buried, mind_rot_buried])]).unwrap();
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE crucible-graveyard candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 3);
    }

    #[test]
    fn oracle_matches_on_a_land_in_the_graveyard_without_crucible() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]), P2]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT).unwrap();
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));
        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST).unwrap();
        let [mind_rot_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(MIND_ROT).unwrap();
        game.play([p1.arrange_graveyard([forest_buried, mind_rot_buried])]).unwrap();
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE graveyard-without-crucible candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_on_an_opponents_graveyard_with_crucible() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([FOREST]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT).unwrap();
        let [forest] = game.player(p2).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *forest, players::PlayerId(*p1)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([
            p1.cast(mind_rot).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert!(game.moved(forest).to(GRAVEYARD).of(p2));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE opponent-graveyard candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 3);
    }

    #[test]
    fn oracle_matches_on_crucible_during_an_opponents_turn() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]),
            P2,
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT).unwrap();
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                PlayDatalog::legality(answers, *mind_rot, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST).unwrap();
        let [mind_rot_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(MIND_ROT).unwrap();
        game.play([p1.arrange_graveyard([forest_buried, mind_rot_buried])]).unwrap();
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));
        let holder = game.priority_player().unwrap();
        let (candidates, datafrog_time) = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                let mut datafrog_time = Duration::ZERO;
                for card in PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let started = Instant::now();
                    let datafrogged = PlayDatalog::legality(answers, card, holder).unwrap();
                    datafrog_time += started.elapsed();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                (candidates, datafrog_time)
            })
            .unwrap();
        eprintln!(
            "ORACLE crucible-opponent-turn candidates={candidates} datafrog_time={datafrog_time:?}"
        );
        assert_eq!(candidates, 3);
    }
}
