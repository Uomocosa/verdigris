use bevy::prelude::Entity;

use crate::events;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Value;

use crate::legality;

pub fn legality(
    answers: &rules::Rules,
    card: Entity,
    player: players::PlayerId,
) -> Result<Option<rules::Legality>, legality::TranslationError> {
    if !legality::PlayDatalog::visible_entities(answers, player).contains(&card)
        || !answers.players_in_game(rules::Players::All, Some(player)).contains(&player)
    {
        return Ok(None);
    }
    let event = events::Play { card, player };
    let program = legality::PlayDatalog::play_program(answers, &event)?;
    let facts = legality::PlayDatalog::snapshot(answers, player);
    let derived = program.evaluate(&facts)?;
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &legality::PlayDatalog::VIOLATES,
            &[
                Value::Sym(legality::PlayDatalog::PLAY),
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
    for change in legality::PlayDatalog::holder_changes(
        answers,
        &event,
        rules::Target::event::<events::Play>(),
    ) {
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
    use crate::GRAVEYARD;
    use crate::HAND;
    use crate::cards;
    use crate::events;
    use crate::mana;
    use crate::players;
    use crate::rules;
    use crate::testing::AZUSA_LOST_BUT_SEEKING;
    use crate::testing::CRUCIBLE_OF_WORLDS;
    use crate::testing::FOREST;
    use crate::testing::GRIZZLY_BEARS;
    use crate::testing::MIND_ROT;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;
    use crate::turns;
    use crate::zones;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;

    use super::legality;

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
                legality::PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest)]).unwrap();
        assert!(game.moved(forest).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE played-land candidates={candidates}");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn oracle_matches_on_a_second_land_in_the_same_turn() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([FOREST, FOREST, GRIZZLY_BEARS]), P2]);
        let [forest_1, forest_2] = game.player(p1).cards_in(HAND).exactly::<2>(FOREST).unwrap();
        let [_bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                legality::PlayDatalog::legality(answers, *forest_1, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest_1)]).unwrap();
        assert!(game.moved(forest_1).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                let hidden = (*forest_2, players::PlayerId(*p2));
                assert_eq!(
                    legality::PlayDatalog::legality(answers, hidden.0, hidden.1),
                    Ok(None)
                );
                candidates
            })
            .unwrap();
        eprintln!("ORACLE second-land candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *forest_1, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.play_land(forest_1)]).unwrap();
        game.play([p1.play_land(forest_2)]).unwrap();
        game.play([p1.play_land(forest_3)]).unwrap();
        assert!(game.moved(forest_3).to(BATTLEFIELD).of(p1));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                assert_eq!(
                    legality::PlayDatalog::legality(answers, *forest_4, players::PlayerId(*p2)),
                    Ok(None)
                );
                candidates
            })
            .unwrap();
        eprintln!("ORACLE azusa candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p2.pass_priority()]).unwrap();
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE opponent-turn candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *mind_rot, players::PlayerId(*p2)),
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
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE crucible-graveyard candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *forest, players::PlayerId(*p2)),
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
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE graveyard-without-crucible candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *forest, players::PlayerId(*p1)),
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
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE opponent-graveyard candidates={candidates}");
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
                legality::PlayDatalog::legality(answers, *mind_rot, players::PlayerId(*p2)),
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
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Play {
                        card,
                        player: holder,
                    });
                    let datafrogged =
                        legality::PlayDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE crucible-opponent-turn candidates={candidates}");
        assert_eq!(candidates, 3);
    }
}
