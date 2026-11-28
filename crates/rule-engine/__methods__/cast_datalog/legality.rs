use bevy::prelude::Entity;

use crate::events;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Value;

use crate::legality;

pub fn legality(
    answers: &rules::Rules,
    spell: Entity,
    caster: players::PlayerId,
) -> Result<Option<rules::Legality>, legality::TranslationError> {
    if !legality::PlayDatalog::visible_entities(answers, caster).contains(&spell)
        || !answers.players_in_game(rules::Players::All, Some(caster)).contains(&caster)
    {
        return Ok(None);
    }
    let event = events::Cast { spell, caster };
    let program = legality::CastDatalog::cast_program(answers, &event)?;
    let facts = legality::CastDatalog::snapshot(answers, caster);
    let derived = program.evaluate(&facts)?;
    let violated = |id: rules::RestrictionId| {
        derived.contains(
            &legality::PlayDatalog::VIOLATES,
            &[
                Value::Sym(legality::CastDatalog::CAST),
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
    for change in legality::PlayDatalog::holder_changes(
        answers,
        &event,
        rules::Target::event::<events::Cast>(),
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

    use crate::HAND;
    use crate::cards;
    use crate::events;
    use crate::mana;
    use crate::players;
    use crate::rules;
    use crate::testing::DISPEL;
    use crate::testing::DIVINATION;
    use crate::testing::FOREST;
    use crate::testing::LEYLINE_OF_ANTICIPATION;
    use crate::testing::LIGHTNING_BOLT;
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
                    legality(&answers, bolt, first).unwrap(),
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
                legality::CastDatalog::legality(answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(bolt)]).unwrap();
        assert!(game.stack().has(LIGHTNING_BOLT));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-instant candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p2)),
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
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-opponent-turn candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(bolt).targeting(p2)]).unwrap();
        assert!(game.stack().has(LIGHTNING_BOLT));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                assert_eq!(
                    legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p2)),
                    Ok(None)
                );
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-stack-nonempty candidates={candidates}");
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_land_proposed_as_a_spell() {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                legality::CastDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-land candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-combat candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p2.pass_priority()]).unwrap();
        game.play([p1.cast(divination)]).unwrap();
        assert!(game.stack().has(DIVINATION));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-leyline-opponent-turn candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.cast(divination)]).unwrap();
        assert!(game.stack().has(DIVINATION));
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-leyline-combat candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *bolt, players::PlayerId(*p2)),
                Ok(None)
            );
            assert_eq!(
                legality::CastDatalog::legality(answers, *dispel, players::PlayerId(*p1)),
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
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-leyline-stack candidates={candidates}");
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
                legality::CastDatalog::legality(answers, *divination, players::PlayerId(*p1)),
                Ok(None)
            );
        })
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let holder = game.priority_player().unwrap();
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-leyline-opponent candidates={candidates}");
        assert_eq!(candidates, 2);
    }

    #[test]
    fn oracle_matches_for_a_land_with_leyline_of_anticipation() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            assert_eq!(
                legality::CastDatalog::legality(answers, *forest, players::PlayerId(*p2)),
                Ok(None)
            );
        })
        .unwrap();
        let holder = game.priority_player().unwrap_or(players::PlayerId(*p1));
        let candidates = game
            .read_rules(|answers, _legal| {
                let mut candidates = 0;
                for card in legality::PlayDatalog::visible_entities(answers, holder) {
                    let expected = answers.legality(&events::Cast {
                        spell: card,
                        caster: holder,
                    });
                    let datafrogged =
                        legality::CastDatalog::legality(answers, card, holder).unwrap();
                    assert_eq!(datafrogged, Some(expected));
                    candidates += 1;
                }
                candidates
            })
            .unwrap();
        eprintln!("ORACLE cast-land-leyline candidates={candidates}");
        assert_eq!(candidates, 2);
    }
}
