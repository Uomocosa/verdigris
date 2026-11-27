use crate::rules;
use crate::rules::traits::event::Event;

pub fn legality<E: Event>(answers: &rules::Rules, event: &E) -> rules::Legality {
    let waived = |restriction: &rules::Restriction| {
        answers.waived(
            event,
            rules::Target::Restriction(restriction.id),
            restriction.unwaivable,
        )
    };
    let added: Vec<rules::Restriction> = answers
        .holder_changes(event, rules::Target::event::<E>())
        .into_iter()
        .filter_map(|change| match change {
            rules::Change::Restrict { restriction, .. } => Some(restriction),
            _ => None,
        })
        .collect();
    let mut reasons: Vec<rules::Why> = E::restrictions()
        .into_iter()
        .chain(added)
        .filter(|restriction| !waived(restriction))
        .filter(|restriction| !answers.eval(&restriction.condition.bind(event), event, None))
        .map(|restriction| rules::Why::Restriction(restriction.id))
        .collect();
    if !answers.performs(event) {
        reasons.push(rules::Why::NotPerformable);
    }
    if reasons.is_empty() {
        rules::Legality::Legal
    } else {
        rules::Legality::Illegal(reasons)
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::events;
    use crate::keywords;
    use crate::rules;
    use crate::testing::*;

    use super::legality;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *bears,
                            player: p1.id
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_tapped_creature_fails_an_untapped_restriction() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *bears,
                            player: p2.id
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(UNTAPPED)])
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_waived_restriction_is_skipped_for_the_waiving_creature() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield_tapped([TEST_RULE_WAIVER]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *waiver,
                            player: p1.id
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_restriction_that_does_not_hold_is_illegal() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([TEST_RULE_RESTRICTION]), P2]);
        let [restriction] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_RESTRICTION)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<RestrictedRule>(
                        &answers,
                        &RestrictedRule {
                            attacker: *restriction,
                            player: p1.id
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-rule-restriction"
                    ),)])
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn an_allowance_satisfies_a_failing_restriction_when_its_alternative_holds() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([
            P1.battlefield([TEST_RULE_ALLOWANCE]),
            P2.battlefield_tapped([WIND_DRAKE]),
        ]);
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.eval(
                    &rules::has_keyword(keywords::flying::ID).of(TestRule::ATTACKER),
                    &TestRule {
                        attacker: *drake,
                        player: p2.id
                    },
                    None,
                ));
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *drake,
                            player: p2.id
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn an_allowance_leaves_a_failing_restriction_when_its_alternative_fails() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([
            P1.battlefield([TEST_RULE_ALLOWANCE]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!answers.eval(
                    &rules::has_keyword(keywords::flying::ID).of(TestRule::ATTACKER),
                    &TestRule {
                        attacker: *bears,
                        player: p2.id
                    },
                    None,
                ));
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *bears,
                            player: p2.id
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(UNTAPPED)])
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn an_unwaivable_restriction_is_not_satisfied_by_as_though() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield_tapped([TEST_RULE_WAIVER]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<AbsoluteRule>(
                        &answers,
                        &AbsoluteRule {
                            attacker: *waiver,
                            player: p1.id
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(UNTAPPED)])
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn an_only_if_for_the_player_that_does_not_hold_is_illegal() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([TEST_PLAYER_RESTRICTION, GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *bears,
                            player: p1.id
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/player-restriction"
                    ),)])
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_gain_life_is_illegal_for_its_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TEST_CANT_DISCARD]), P2]);
        game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_CANT_DISCARD)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::GainLife>(
                        &answers,
                        &events::GainLife {
                            player: p1.id,
                            amount: 3
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-cant-discard"
                    ),)])
                );
                assert_eq!(
                    legality::<events::GainLife>(
                        &answers,
                        &events::GainLife {
                            player: p2.id,
                            amount: 3
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_mill_is_illegal_for_its_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TEST_CANT_DISCARD]), P2]);
        game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_CANT_DISCARD)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::Mill>(
                        &answers,
                        &events::Mill {
                            player: p1.id,
                            amount: 5,
                            cause: rules::EventId("608.2/resolve"),
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-cant-discard"
                    ),)])
                );
                assert_eq!(
                    legality::<events::Mill>(
                        &answers,
                        &events::Mill {
                            player: p2.id,
                            amount: 5,
                            cause: rules::EventId("608.2/resolve"),
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_create_tokens_is_illegal_for_its_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TEST_CANT_DISCARD]), P2]);
        game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_CANT_DISCARD)?;
        game.pass_priority();
        let own_token = game.app.world_mut().spawn(cards::Controller(p1.id)).id();
        let other_token = game.app.world_mut().spawn(cards::Controller(p2.id)).id();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::Create>(&answers, &events::Create { token: own_token }),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-cant-discard"
                    ),)])
                );
                assert_eq!(
                    legality::<events::Create>(&answers, &events::Create { token: other_token }),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_gain_life_leaves_lose_life_legal() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([TEST_CANT_DISCARD]), P2]);
        game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_CANT_DISCARD)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::GainLife>(
                        &answers,
                        &events::GainLife {
                            player: p1.id,
                            amount: 3
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-cant-discard"
                    ),)])
                );
                assert_eq!(
                    legality::<events::LoseLife>(
                        &answers,
                        &events::LoseLife {
                            player: p1.id,
                            amount: 3
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_destroy_is_illegal_for_the_protected_creature() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_SAFE_HAVEN, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [protected] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [exposed] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::Destroy>(
                        &answers,
                        &events::Destroy {
                            permanent: *protected,
                            cause: crate::RESOLUTION_CAUSE,
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-safe-haven"
                    ),)])
                );
                assert_eq!(
                    legality::<events::Destroy>(
                        &answers,
                        &events::Destroy {
                            permanent: *exposed,
                            cause: crate::RESOLUTION_CAUSE,
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_counter_is_illegal_for_the_protected_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_SAFE_HAVEN, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [protected] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [exposed] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::Counter>(
                        &answers,
                        &events::Counter {
                            spell: *protected,
                            cause: crate::RESOLUTION_CAUSE,
                        },
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-safe-haven"
                    ),)])
                );
                assert_eq!(
                    legality::<events::Counter>(
                        &answers,
                        &events::Counter {
                            spell: *exposed,
                            cause: crate::RESOLUTION_CAUSE,
                        },
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_enter_battlefield_is_illegal_for_the_protected_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_NO_TOKENS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [protected] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::EnterTheBattlefield>(
                        &answers,
                        &events::EnterTheBattlefield {
                            permanent: *protected,
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-no-tokens"
                    ),)])
                );
            })
            .unwrap();
        let protected_entity = *protected;
        game.app
            .world_mut()
            .run_system_once(move |mut commands: bevy::prelude::Commands| {
                commands.entity(protected_entity).insert(cards::Controller(p2.id));
            })
            .unwrap();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::EnterTheBattlefield>(
                        &answers,
                        &events::EnterTheBattlefield {
                            permanent: *protected,
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_cant_aimed_at_deal_damage_is_illegal_for_the_protected_recipient() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_SAFE_HAVEN, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [protected] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [exposed] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::DealDamage>(
                        &answers,
                        &events::DealDamage {
                            source: *exposed,
                            recipient: *protected,
                            amount: 3,
                        }
                    ),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(rules::RestrictionId(
                        "test/test-safe-haven"
                    ),)])
                );
                assert_eq!(
                    legality::<events::DealDamage>(
                        &answers,
                        &events::DealDamage {
                            source: *exposed,
                            recipient: *exposed,
                            amount: 3,
                        }
                    ),
                    rules::Legality::Legal
                );
                assert_eq!(
                    legality::<events::DealDamage>(
                        &answers,
                        &events::DealDamage {
                            source: *protected,
                            recipient: *exposed,
                            amount: 3,
                        }
                    ),
                    rules::Legality::Legal
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_source_that_would_deal_zero_damage_deals_no_damage_at_all() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let zero = events::DealDamage {
                    source: *bears,
                    recipient: *p1,
                    amount: 0,
                };
                let one = events::DealDamage { amount: 1, ..zero };
                assert_eq!(
                    legality(&answers, &zero),
                    rules::Legality::Illegal(vec![rules::Why::Restriction(
                        events::deal_damage::NONZERO_DAMAGE
                    )])
                );
                assert_eq!(legality(&answers, &one), rules::Legality::Legal);
            })
            .unwrap();
        Ok(())
    }
}
