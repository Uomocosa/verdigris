use crate::rules;
use crate::rules::traits::event::Event;

pub fn legality<E: Event>(answers: &rules::Rules, event: &E) -> rules::Legality {
    let subject = event.subject();
    let other = event.other();
    let player = event.player();
    let waived = |restriction: &rules::Restriction| {
        if restriction.unwaivable {
            return false;
        }
        let entity = match restriction.role {
            rules::Role::Subject => Some(subject),
            rules::Role::Other => other,
            rules::Role::Player => Some(*player),
        };
        entity.is_some_and(|entity| {
            answers.changes(entity, rules::Target::Restriction(restriction.id)).iter().any(
                |change| match change {
                    rules::Change::AsThough => true,
                    rules::Change::Allow(alternative) => {
                        answers.eval(alternative, subject, other, Some(*player))
                    }
                    _ => false,
                },
            )
        })
    };
    let added: Vec<rules::Restriction> = [
        (rules::Role::Subject, Some(subject)),
        (rules::Role::Other, other),
        (rules::Role::Player, Some(*player)),
    ]
    .into_iter()
    .filter_map(|(role, entity)| entity.map(|entity| (role, entity)))
    .flat_map(|(role, entity)| {
        answers.changes(entity, rules::Target::event::<E>()).into_iter().filter_map(move |change| {
            match change {
                rules::Change::Restrict {
                    holder,
                    restriction,
                } if holder == role => Some(restriction),
                _ => None,
            }
        })
    })
    .collect();
    let base_and_added_hold =
        E::restrictions().into_iter().chain(added).filter(|restriction| !waived(restriction)).all(
            |restriction| {
                answers.eval(
                    &restriction.condition.bind(event),
                    subject,
                    other,
                    Some(*player),
                )
            },
        );
    if base_and_added_hold && answers.performs(event) {
        rules::Legality::Legal
    } else {
        rules::Legality::Illegal
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::effects;
    use crate::events;
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
                    rules::Legality::Illegal
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
                    rules::Legality::Illegal
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
                    &rules::has::<cards::Flying>(),
                    *drake,
                    None,
                    Some(p2.into())
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
                    &rules::has::<cards::Flying>(),
                    *bears,
                    None,
                    Some(p2.into())
                ));
                assert_eq!(
                    legality::<TestRule>(
                        &answers,
                        &TestRule {
                            attacker: *bears,
                            player: p2.id
                        }
                    ),
                    rules::Legality::Illegal
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
                    rules::Legality::Illegal
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
                    rules::Legality::Illegal
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
                    rules::Legality::Illegal
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
                            amount: 5
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::Mill>(
                        &answers,
                        &events::Mill {
                            player: p2.id,
                            amount: 5
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
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::CreateTokens>(
                        &answers,
                        &events::CreateTokens {
                            controller: p1.id,
                            amount: 2,
                            token: effects::TokenDefinition::from(SOLDIER_TOKEN.definition),
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::CreateTokens>(
                        &answers,
                        &events::CreateTokens {
                            controller: p2.id,
                            amount: 2,
                            token: effects::TokenDefinition::from(SOLDIER_TOKEN.definition),
                        }
                    ),
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
                    rules::Legality::Illegal
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
                            controller: p1.id,
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::Destroy>(
                        &answers,
                        &events::Destroy {
                            permanent: *exposed,
                            controller: p2.id,
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
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    legality::<events::Counter>(
                        &answers,
                        &events::Counter {
                            spell: *protected,
                            controller: p1.id,
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::Counter>(
                        &answers,
                        &events::Counter {
                            spell: *protected,
                            controller: p2.id,
                        }
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
                    legality::<events::EnterBattlefield>(
                        &answers,
                        &events::EnterBattlefield {
                            permanent: *protected,
                            controller: p1.id,
                            summoning_sick: true,
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::EnterBattlefield>(
                        &answers,
                        &events::EnterBattlefield {
                            permanent: *protected,
                            controller: p2.id,
                            summoning_sick: true,
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
                            controller: p1.id,
                        }
                    ),
                    rules::Legality::Illegal
                );
                assert_eq!(
                    legality::<events::DealDamage>(
                        &answers,
                        &events::DealDamage {
                            source: *exposed,
                            recipient: *exposed,
                            amount: 3,
                            controller: p2.id,
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
                            controller: p2.id,
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
                    controller: p1.id,
                };
                let one = events::DealDamage { amount: 1, ..zero };
                assert_eq!(legality(&answers, &zero), rules::Legality::Illegal);
                assert_eq!(legality(&answers, &one), rules::Legality::Legal);
            })
            .unwrap();
        Ok(())
    }
}
