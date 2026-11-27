use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event_args::EventArgs;
use crate::zones;

pub fn eval<E: EventArgs>(
    answers: &rules::Rules,
    condition: &rules::Condition,
    event: &E,
    viewpoint: Option<Entity>,
) -> bool {
    let entity = |reference: &rules::ArgRef| match answers.resolve(*reference, event) {
        Some(rules::ArgValue::Entity(found)) => Some(found),
        Some(rules::ArgValue::Player(player)) => Some(*player),
        Some(rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_)) | None => None,
    };
    let amount = |id: &rules::ArgId| match event.arg(*id) {
        Some(rules::ArgValue::Amount(found)) => Some(found),
        Some(rules::ArgValue::Entities(found)) => found.len().try_into().ok(),
        Some(_) | None => None,
    };
    match condition {
        rules::Condition::Has(component) => answers.eval_has(component, viewpoint),
        rules::Condition::HasKeyword(id) => answers.eval_has_keyword(*id, viewpoint),
        rules::Condition::PointsTo(component, reference) => {
            answers.eval_points_to(component, *reference, event, viewpoint)
        }
        rules::Condition::Is(reference) => entity(reference) == viewpoint,
        rules::Condition::CardType(card) => answers.eval_card_type(*card, viewpoint),
        rules::Condition::Color(color) => answers.eval_color(*color, viewpoint),
        rules::Condition::Subtype(subtype) => answers.eval_subtype(*subtype, viewpoint),
        rules::Condition::Supertype(supertype) => answers.eval_supertype(*supertype, viewpoint),
        rules::Condition::Spell => answers.eval_spell(viewpoint),
        rules::Condition::ControlledBy(set) => answers.eval_controlled_by(*set, event, viewpoint),
        rules::Condition::Enchanted => answers.eval_enchanted(event, viewpoint),
        rules::Condition::Characteristic(characteristic, comparison) => {
            answers.eval_characteristic(*characteristic, *comparison, viewpoint)
        }
        rules::Condition::PlayerValue(quantity, comparison) => {
            answers.eval_player_value(*quantity, *comparison, viewpoint)
        }
        rules::Condition::Of(reference, inner) => {
            entity(reference).is_some_and(|candidate| eval(answers, inner, event, Some(candidate)))
        }
        rules::Condition::Each(set, inner) => answers.eval_each(*set, inner, event),
        rules::Condition::AnyPlayer(set, inner) => answers.eval_any_player(*set, inner, event),
        rules::Condition::All(conditions) => {
            conditions.iter().all(|inner| eval(answers, inner, event, viewpoint))
        }
        rules::Condition::Any(conditions) => {
            conditions.iter().any(|inner| eval(answers, inner, event, viewpoint))
        }
        rules::Condition::Not(inner) => !eval(answers, inner, event, viewpoint),
        rules::Condition::True => true,
        rules::Condition::InZone(zone, reference) => {
            answers.eval_in_zone(*zone, *reference, event, viewpoint)
        }
        rules::Condition::StepIs(step) => {
            answers.state.single().is_ok_and(|state| state.turn.step == *step)
        }
        rules::Condition::StackEmpty => answers
            .zones
            .find(zones::SharedZone::Stack, None)
            .is_none_or(|stack| !answers.in_zone.iter().any(|in_zone| **in_zone == stack)),
        rules::Condition::ActivePlayer(reference) => entity(reference).is_some_and(|holder| {
            answers.state.single().is_ok_and(|state| *state.turn.active == holder)
        }),
        rules::Condition::HasPriority(reference) => entity(reference).is_some_and(|holder| {
            answers
                .state
                .single()
                .is_ok_and(|state| state.priority.holder.is_some_and(|id| *id == holder))
        }),
        rules::Condition::ArgAmount(id, comparison) | rules::Condition::Count(id, comparison) => {
            amount(id).is_some_and(|value| {
                i32::try_from(value).is_ok_and(|narrow| comparison.holds(narrow))
            })
        }
        rules::Condition::ArgAtLeastRemainingToughness(id) => viewpoint.is_some_and(|subject| {
            amount(id).is_some_and(|found| found >= answers.remaining_toughness(subject))
        }),
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::counters;
    use crate::events;
    use crate::keywords;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    use super::eval;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let tapped = TestRule {
            attacker: *bears,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(
                    &answers,
                    &rules::has::<cards::Tapped>().of(TestRule::ATTACKER),
                    &tapped,
                    None,
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn has_tapped_is_false_on_an_untapped_creature() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(
                    &answers,
                    &rules::has::<cards::Tapped>().of(TestRule::ATTACKER),
                    &attack,
                    None,
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn awake_and_airborne_holds_for_an_untapped_flyer() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([WIND_DRAKE]), P2]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        let attack = TestRule {
            attacker: *drake,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let awake_and_airborne = (!rules::has::<cards::Tapped>()
                    & rules::has_keyword(keywords::flying::ID))
                .of(TestRule::ATTACKER);
                assert!(eval(&answers, &awake_and_airborne, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn awake_and_airborne_fails_for_a_tapped_flyer() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([WIND_DRAKE])]);
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        let attack = TestRule {
            attacker: *drake,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let awake_and_airborne = (!rules::has::<cards::Tapped>()
                    & rules::has_keyword(keywords::flying::ID))
                .of(TestRule::ATTACKER);
                assert!(!eval(&answers, &awake_and_airborne, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn tapped_or_flying_matches_either_side() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [resting] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [tapped] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let rest = TestRule {
            attacker: *resting,
            player: p1.id,
        };
        let tap = TestRule {
            attacker: *tapped,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let tapped_or_flying = (rules::has::<cards::Tapped>()
                    | rules::has_keyword(keywords::flying::ID))
                .of(TestRule::ATTACKER);
                assert!(!eval(&answers, &tapped_or_flying, &rest, None));
                assert!(eval(&answers, &tapped_or_flying, &tap, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn power_at_least_three_rejects_two_and_accepts_three() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2.battlefield([WATCHWOLF])]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [wolf] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WATCHWOLF)?;
        game.app.update();
        let small = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        let big_attack = TestRule {
            attacker: *wolf,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let big = rules::power().of(TestRule::ATTACKER).is(rules::Value(3).or_more());
                assert!(!eval(&answers, &big, &small, None));
                assert!(eval(&answers, &big, &big_attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn other_condition_reads_the_other_entity() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([WIND_DRAKE]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        let bears_blocked = events::DeclareBlocker {
            blocker: *bears,
            attacker: *drake,
            player: p2.id,
        };
        let drake_blocked = events::DeclareBlocker {
            blocker: *drake,
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let attacker_flies =
                    rules::has_keyword(keywords::flying::ID).of(events::DeclareBlocker::ATTACKER);
                assert!(eval(&answers, &attacker_flies, &bears_blocked, None));
                assert!(!eval(&answers, &attacker_flies, &drake_blocked, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn other_condition_without_other_is_false() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([WIND_DRAKE]), P2]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        let attack = TestRule {
            attacker: *drake,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let missing = rules::has_keyword(keywords::flying::ID).of(rules::ArgId("missing"));
                assert!(!eval(&answers, &missing, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_owner_matches_the_ctx_player() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let owned_by_player =
                    rules::points_to::<cards::Owner>(TestRule::PLAYER).of(TestRule::ATTACKER);
                assert!(eval(&answers, &owned_by_player, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_owner_is_false_for_another_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let owned_by_player =
                    rules::points_to::<cards::Owner>(TestRule::PLAYER).of(TestRule::ATTACKER);
                assert!(!eval(&answers, &owned_by_player, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_without_the_component_is_false() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let blocking_player =
                    rules::points_to::<cards::Blocking>(TestRule::PLAYER).of(TestRule::ATTACKER);
                assert!(!eval(&answers, &blocking_player, &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn card_type_matches_a_land_and_rejects_a_creature() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST, GRIZZLY_BEARS]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let cast_forest = TestRule {
            attacker: *forest,
            player: p1.id,
        };
        let cast_bears = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let is_land = rules::card_type(cards::CardType::Land).of(TestRule::ATTACKER);
                assert!(eval(&answers, &is_land, &cast_forest, None));
                assert!(!eval(&answers, &is_land, &cast_bears, None));
                let is_creature =
                    rules::card_type(cards::CardType::Creature).of(TestRule::ATTACKER);
                assert!(!eval(&answers, &is_creature, &cast_forest, None));
                assert!(eval(&answers, &is_creature, &cast_bears, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn of_player_evaluates_on_the_player() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let on_player = rules::Condition::True.of(TestRule::PLAYER);
                assert!(eval(&answers, &on_player, &attack, None));
                let missing = rules::Condition::True.of(rules::ArgId("missing"));
                assert!(!eval(&answers, &missing, &attack, None));
                assert!(eval(
                    &answers,
                    &rules::life().of(TestRule::PLAYER).is(rules::Value(20).or_more()),
                    &attack,
                    None
                ));
                assert!(!eval(
                    &answers,
                    &rules::life().of(TestRule::PLAYER).is(rules::Value(21).or_more()),
                    &attack,
                    None
                ));
                assert!(eval(
                    &answers,
                    &rules::counters(counters::POISON)
                        .of(TestRule::PLAYER)
                        .is(rules::Value(0).or_less()),
                    &attack,
                    None,
                ));
                assert!(eval(
                    &answers,
                    &rules::cards_in_hand().of(TestRule::PLAYER).is(rules::Value(0).or_more()),
                    &attack,
                    None,
                ));
                assert!(!eval(
                    &answers,
                    &rules::cards_in_hand().of(TestRule::PLAYER).is(rules::Value(1000).or_more()),
                    &attack,
                    None,
                ));
            })
            .unwrap();
        let context = events::AbilityContext {
            ability: *bears,
            source: *bears,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(
                    &answers,
                    &rules::life()
                        .is(rules::Value(20).or_more())
                        .of(rules::each(rules::Players::Opponents)),
                    &context,
                    None,
                ));
                assert!(!eval(
                    &answers,
                    &rules::life()
                        .is(rules::Value(21).or_more())
                        .of(rules::each(rules::Players::Opponents)),
                    &context,
                    None,
                ));
                assert!(eval(
                    &answers,
                    &rules::life()
                        .is(rules::Value(20).or_less())
                        .of(rules::an(rules::Players::All)),
                    &context,
                    None,
                ));
                assert!(!eval(
                    &answers,
                    &rules::life()
                        .is(rules::Value(19).or_less())
                        .of(rules::an(rules::Players::Opponents)),
                    &context,
                    None,
                ));
                assert!(eval(
                    &answers,
                    &rules::life()
                        .is(rules::Value(20).or_more())
                        .of(rules::each(rules::Players::You)),
                    &context,
                    None,
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn in_zone_matches_zone_and_owner() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2.hand([GRIZZLY_BEARS])]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let own = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        let foreign = TestRule {
            attacker: *bears,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let on_own_battlefield =
                    rules::in_zone(BATTLEFIELD, TestRule::PLAYER).of(TestRule::ATTACKER);
                assert!(eval(&answers, &on_own_battlefield, &own, None));
                assert!(!eval(&answers, &on_own_battlefield, &foreign, None));
                let in_own_hand = rules::in_zone(HAND, TestRule::PLAYER).of(TestRule::ATTACKER);
                assert!(!eval(&answers, &in_own_hand, &own, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn step_is_matches_the_current_step() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(
                    &answers,
                    &rules::step_is(turns::Step::Upkeep),
                    &attack,
                    None,
                ));
                assert!(!eval(
                    &answers,
                    &rules::step_is(turns::Step::PrecombatMain),
                    &attack,
                    None,
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn main_phase_holds_during_either_main_phase() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(&answers, &rules::main_phase(), &attack, None));
            })
            .unwrap();
        let (mut upkeep, [q1, _q2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [resting] = upkeep.player(q1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let upkeep_attack = TestRule {
            attacker: *resting,
            player: q1.id,
        };
        upkeep
            .app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(&answers, &rules::main_phase(), &upkeep_attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn stack_empty_holds_when_nothing_is_on_the_stack() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(&answers, &rules::stack_empty(), &attack, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn stack_empty_fails_while_a_spell_is_on_the_stack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2)])?;
        let [stacked] = game.stack().exactly::<1>(LIGHTNING_BOLT)?;
        let cast = TestRule {
            attacker: *stacked,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(&answers, &rules::stack_empty(), &cast, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn active_player_matches_the_turns_active_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let own = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        let foreign = TestRule {
            attacker: *bears,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let active = rules::active_player(TestRule::PLAYER);
                assert!(eval(&answers, &active, &own, None));
                assert!(!eval(&answers, &active, &foreign, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn has_priority_matches_the_priority_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app.update();
        let own = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        let foreign = TestRule {
            attacker: *bears,
            player: p2.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let priority = rules::has_priority(TestRule::PLAYER);
                assert!(eval(&answers, &priority, &own, None));
                assert!(!eval(&answers, &priority, &foreign, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn operators_flatten_nested_combinators() {
        let tapped = rules::has::<cards::Tapped>();
        let flying = rules::has_keyword(keywords::flying::ID);
        let reach = rules::has_keyword(keywords::reach::ID);
        let anded = tapped.clone() & flying.clone() & reach.clone();
        assert_eq!(
            anded,
            rules::Condition::All(vec![tapped.clone(), flying.clone(), reach.clone()])
        );
        let ored = tapped.clone() | flying.clone() | reach.clone();
        assert_eq!(
            ored,
            rules::Condition::Any(vec![tapped.clone(), flying, reach.clone()])
        );
        let negated = !tapped.clone();
        assert_eq!(negated, rules::Condition::Not(Box::new(tapped.clone())));
        assert_eq!(
            tapped.clone() & rules::Condition::True,
            rules::Condition::All(vec![tapped, rules::Condition::True])
        );
        assert_eq!(reach, rules::has_keyword(keywords::reach::ID));
    }
}
