use bevy::prelude::Entity;

use crate::cards;
use crate::rules;

pub fn eval(
    answers: &rules::Rules,
    predicate: &rules::Predicate,
    subject: Entity,
    other: Option<Entity>,
) -> bool {
    match predicate {
        rules::Predicate::Has(key) => answers
            .registry
            .single()
            .ok()
            .and_then(|registry| registry.component_id(*key))
            .is_some_and(|component| {
                answers.entities.get(subject).is_ok_and(|entity| entity.contains_id(component))
            }),
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
            rules::Role::Subject => eval(answers, inner, subject, other),
            rules::Role::Other => {
                other.is_some_and(|candidate| eval(answers, inner, candidate, Some(subject)))
            }
        },
        rules::Predicate::All(predicates) => {
            predicates.iter().all(|inner| eval(answers, inner, subject, other))
        }
        rules::Predicate::Any(predicates) => {
            predicates.iter().any(|inner| eval(answers, inner, subject, other))
        }
        rules::Predicate::Not(inner) => !eval(answers, inner, subject, other),
        rules::Predicate::True => true,
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval(&answers, &rules::has::<cards::Tapped>(), *bears, None));
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
                assert!(eval(&answers, &awake_and_airborne, *drake, None));
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
                assert!(!eval(&answers, &awake_and_airborne, *drake, None));
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
                assert!(!eval(&answers, &tapped_or_flying, *resting, None));
                assert!(eval(&answers, &tapped_or_flying, *tapped, None));
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
                assert!(!eval(&answers, &big, *bears, None));
                assert!(eval(&answers, &big, *wolf, None));
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
                assert!(!eval(&answers, &other_flies, *bears, None));
                assert!(eval(&answers, &other_flies, *bears, Some(*drake)));
                assert!(!eval(&answers, &other_flies, *drake, Some(*bears)));
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
                assert!(!eval(&answers, &other_flies, *drake, None));
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
