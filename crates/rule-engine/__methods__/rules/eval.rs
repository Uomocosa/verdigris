use bevy::prelude::Entity;

use crate::cards;
use crate::rules;
use crate::zones;

pub fn eval(
    answers: &rules::Rules,
    predicate: &rules::Predicate,
    subject: Entity,
    other: Option<Entity>,
    player: Option<Entity>,
) -> bool {
    let target = |role: &rules::Role| match role {
        rules::Role::Subject => Some(subject),
        rules::Role::Other => other,
        rules::Role::Player => player,
    };
    match predicate {
        rules::Predicate::Has(key) => answers
            .registry
            .single()
            .ok()
            .and_then(|registry| registry.component_id(*key))
            .is_some_and(|component| {
                answers.entities.get(subject).is_ok_and(|entity| entity.contains_id(component))
            }),
        rules::Predicate::PointsTo(key, role) => target(role).is_some_and(|wanted| {
            answers
                .registry
                .single()
                .ok()
                .and_then(|registry| {
                    answers
                        .entities
                        .get(subject)
                        .ok()
                        .and_then(|entity| registry.target_of(*key, entity))
                })
                .is_some_and(|found| found == wanted)
        }),
        rules::Predicate::CardType(card) => answers
            .definitions
            .get(subject)
            .is_ok_and(|definition| definition.type_line.types.contains(card)),
        rules::Predicate::Characteristic(characteristic, comparison, threshold) => answers
            .entities
            .get(subject)
            .ok()
            .and_then(|entity| entity.get::<cards::EffectivePT>())
            .is_some_and(|effective| {
                let value = match characteristic {
                    rules::Characteristic::Power => effective.power,
                    rules::Characteristic::Toughness => effective.toughness,
                };
                match comparison {
                    rules::Comparison::AtLeast => value >= *threshold,
                    rules::Comparison::AtMost => value <= *threshold,
                    rules::Comparison::EqualTo => value == *threshold,
                }
            }),
        rules::Predicate::Of(role, inner) => match role {
            rules::Role::Subject => eval(answers, inner, subject, other, player),
            rules::Role::Other => other
                .is_some_and(|candidate| eval(answers, inner, candidate, Some(subject), player)),
            rules::Role::Player => {
                player.is_some_and(|candidate| eval(answers, inner, candidate, other, player))
            }
        },
        rules::Predicate::All(predicates) => {
            predicates.iter().all(|inner| eval(answers, inner, subject, other, player))
        }
        rules::Predicate::Any(predicates) => {
            predicates.iter().any(|inner| eval(answers, inner, subject, other, player))
        }
        rules::Predicate::Not(inner) => !eval(answers, inner, subject, other, player),
        rules::Predicate::True => true,
        rules::Predicate::InZone(zone, role) => target(role).is_some_and(|holder| {
            answers
                .entities
                .get(subject)
                .ok()
                .and_then(|entity| entity.get::<zones::InZone>())
                .is_some_and(|in_zone| {
                    answers.zones.get(**in_zone).is_ok_and(|(_, found, owner)| {
                        *found == *zone && owner.is_some_and(|owner| ***owner == holder)
                    })
                })
        }),
        rules::Predicate::StepIs(step) => {
            answers.turn.single().is_ok_and(|turn| turn.step == *step)
        }
        rules::Predicate::StackEmpty => answers
            .zones
            .find(zones::SharedZone::Stack, None)
            .is_none_or(|stack| !answers.in_zone.iter().any(|in_zone| **in_zone == stack)),
        rules::Predicate::ActivePlayer(role) => target(role)
            .is_some_and(|holder| answers.turn.single().is_ok_and(|turn| *turn.active == holder)),
        rules::Predicate::HasPriority(role) => target(role).is_some_and(|holder| {
            answers.priority.single().is_ok_and(|priority| *priority.player == holder)
        }),
        rules::Predicate::EventField(..) => false,
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    use super::eval;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(
                    &answers,
                    &rules::has::<cards::Tapped>(),
                    *bears,
                    None,
                    None
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn has_tapped_is_false_on_an_untapped_creature() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(
                    &answers,
                    &rules::has::<cards::Tapped>(),
                    *bears,
                    None,
                    None
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let awake_and_airborne =
                    !rules::has::<cards::Tapped>() & rules::has::<cards::Flying>();
                assert!(eval(&answers, &awake_and_airborne, *drake, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn awake_and_airborne_fails_for_a_tapped_flyer() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([WIND_DRAKE])]);
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let awake_and_airborne =
                    !rules::has::<cards::Tapped>() & rules::has::<cards::Flying>();
                assert!(!eval(&answers, &awake_and_airborne, *drake, None, None));
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let tapped_or_flying =
                    rules::has::<cards::Tapped>() | rules::has::<cards::Flying>();
                assert!(!eval(&answers, &tapped_or_flying, *resting, None, None));
                assert!(eval(&answers, &tapped_or_flying, *tapped, None, None));
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let big = rules::at_least(rules::Characteristic::Power, 3);
                assert!(!eval(&answers, &big, *bears, None, None));
                assert!(eval(&answers, &big, *wolf, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn other_predicate_reads_the_other_entity() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([WIND_DRAKE]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let other_flies = rules::other(rules::has::<cards::Flying>());
                assert!(!eval(&answers, &other_flies, *bears, None, None));
                assert!(eval(&answers, &other_flies, *bears, Some(*drake), None));
                assert!(!eval(&answers, &other_flies, *drake, Some(*bears), None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn other_predicate_without_other_is_false() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([WIND_DRAKE]), P2]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let other_flies = rules::other(rules::has::<cards::Flying>());
                assert!(!eval(&answers, &other_flies, *drake, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_owner_matches_the_ctx_player() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let owned_by_player = rules::points_to::<cards::Owner>(rules::Role::Player);
                assert!(eval(
                    &answers,
                    &owned_by_player,
                    *bears,
                    None,
                    Some(p1.into())
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_owner_is_false_for_another_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let owned_by_player = rules::points_to::<cards::Owner>(rules::Role::Player);
                assert!(!eval(
                    &answers,
                    &owned_by_player,
                    *bears,
                    None,
                    Some(p2.into())
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn points_to_without_the_component_is_false() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let blocking_player = rules::points_to::<cards::Blocking>(rules::Role::Player);
                assert!(!eval(
                    &answers,
                    &blocking_player,
                    *bears,
                    None,
                    Some(p1.into())
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn card_type_matches_a_land_and_rejects_a_creature() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST, GRIZZLY_BEARS]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let is_land = rules::card_type(cards::CardType::Land);
                assert!(eval(&answers, &is_land, *forest, None, None));
                assert!(!eval(&answers, &is_land, *bears, None, None));
                let is_creature = rules::card_type(cards::CardType::Creature);
                assert!(!eval(&answers, &is_creature, *forest, None, None));
                assert!(eval(&answers, &is_creature, *bears, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn of_player_evaluates_on_the_player() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let on_player =
                    rules::Predicate::Of(rules::Role::Player, Box::new(rules::Predicate::True));
                assert!(eval(&answers, &on_player, *bears, None, Some(p1.into())));
                assert!(!eval(&answers, &on_player, *bears, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn in_zone_matches_zone_and_owner() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2.hand([GRIZZLY_BEARS])]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let on_own_battlefield = rules::in_zone(BATTLEFIELD, rules::Role::Player);
                assert!(eval(
                    &answers,
                    &on_own_battlefield,
                    *bears,
                    None,
                    Some(p1.into())
                ));
                assert!(!eval(
                    &answers,
                    &on_own_battlefield,
                    *bears,
                    None,
                    Some(p2.into())
                ));
                let in_own_hand = rules::in_zone(HAND, rules::Role::Player);
                assert!(!eval(&answers, &in_own_hand, *bears, None, Some(p1.into())));
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(
                    &answers,
                    &rules::step_is(turns::Step::Upkeep),
                    *bears,
                    None,
                    None
                ));
                assert!(!eval(
                    &answers,
                    &rules::step_is(turns::Step::PrecombatMain),
                    *bears,
                    None,
                    None
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn main_phase_holds_during_either_main_phase() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(&answers, &rules::main_phase(), *bears, None, None));
            })
            .unwrap();
        let (mut upkeep, [q1, _q2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [resting] = upkeep.player(q1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        upkeep
            .app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(&answers, &rules::main_phase(), *resting, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn stack_empty_holds_when_nothing_is_on_the_stack() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(&answers, &rules::stack_empty(), *bears, None, None));
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval(&answers, &rules::stack_empty(), *stacked, None, None));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn active_player_matches_the_turns_active_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let active = rules::active_player(rules::Role::Player);
                assert!(eval(&answers, &active, *bears, None, Some(p1.into())));
                assert!(!eval(&answers, &active, *bears, None, Some(p2.into())));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn has_priority_matches_the_priority_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let priority = rules::has_priority(rules::Role::Player);
                assert!(eval(&answers, &priority, *bears, None, Some(p1.into())));
                assert!(!eval(&answers, &priority, *bears, None, Some(p2.into())));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn operators_flatten_nested_combinators() {
        let tapped = rules::has::<cards::Tapped>();
        let flying = rules::has::<cards::Flying>();
        let reach = rules::has::<cards::Reach>();
        let anded = tapped.clone() & flying.clone() & reach.clone();
        assert_eq!(
            anded,
            rules::Predicate::All(vec![tapped.clone(), flying.clone(), reach.clone()])
        );
        let ored = tapped.clone() | flying.clone() | reach.clone();
        assert_eq!(
            ored,
            rules::Predicate::Any(vec![tapped.clone(), flying, reach.clone()])
        );
        let negated = !tapped.clone();
        assert_eq!(negated, rules::Predicate::Not(Box::new(tapped.clone())));
        assert_eq!(
            tapped.clone() & rules::Predicate::True,
            rules::Predicate::All(vec![tapped, rules::Predicate::True])
        );
        assert_eq!(reach, rules::has::<cards::Reach>());
    }
}
