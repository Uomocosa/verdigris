use bevy::prelude::Entity;

use crate::actions;
use crate::events;
use crate::rules;

pub fn candidates(
    choice: &actions::PickChoice,
    answers: &rules::Rules,
) -> Vec<(Entity, rules::Legality)> {
    match &choice.among {
        actions::Candidates::Fixed(set) => {
            set.iter().map(|entity| (*entity, rules::Legality::Legal)).collect()
        }
        actions::Candidates::Matching {
            filter,
            context,
            check,
        } => {
            let mut listed: Vec<Entity> = answers.visible_entities(choice.player);
            listed.extend(
                answers.players_in_game(rules::Players::All, None).iter().map(|player| **player),
            );
            listed.sort();
            listed.dedup();
            listed
                .into_iter()
                .map(|candidate| {
                    let mut reasons: Vec<rules::Why> = Vec::new();
                    if !answers.eval(filter, context, Some(candidate)) {
                        reasons.push(rules::Why::NotMatching);
                    }
                    match check {
                        actions::CandidateCheck::None => {}
                        actions::CandidateCheck::BecomeTarget => {
                            if let rules::Legality::Illegal(whys) =
                                answers.legality(&events::BecomeTarget {
                                    target: candidate,
                                    spell_or_ability: context.spell_or_ability,
                                })
                            {
                                reasons.extend(whys);
                            }
                        }
                        actions::CandidateCheck::Discard { cause } => {
                            if let rules::Legality::Illegal(whys) =
                                answers.legality(&events::Discard {
                                    cards: vec![candidate],
                                    cause: *cause,
                                })
                            {
                                reasons.extend(whys);
                            }
                        }
                    }
                    let legality = if reasons.is_empty() {
                        rules::Legality::Legal
                    } else {
                        rules::Legality::Illegal(reasons)
                    };
                    (candidate, legality)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::effects;
    use crate::events;
    use crate::keywords;
    use crate::rules;
    use crate::testing::*;

    use super::candidates;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, FOREST]),
            P2.battlefield([GLADECOVER_SCOUT]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        let [scout] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GLADECOVER_SCOUT)?;
        let fixed = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([*bears, *forest])),
            count: actions::Count::exactly(2),
            ordered: false,
        };
        let matching = actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Matching {
                filter: effects::creature(),
                context: events::AbilityContext {
                    spell_or_ability: *bears,
                },
                check: actions::CandidateCheck::None,
            },
            count: actions::Count::up_to(2),
            ordered: false,
        };
        let targeted = actions::PickChoice {
            id: actions::ChoiceId("may"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Matching {
                filter: effects::creature(),
                context: events::AbilityContext {
                    spell_or_ability: *bears,
                },
                check: actions::CandidateCheck::BecomeTarget,
            },
            count: actions::Count::up_to(2),
            ordered: false,
        };
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = candidates(&fixed, &answers);
                assert_eq!(listed.len(), 2);
                assert!(listed.iter().all(|(_, legality)| *legality == rules::Legality::Legal));
                assert!(listed.iter().any(|(entity, _)| *entity == *bears));
                assert!(listed.iter().any(|(entity, _)| *entity == *forest));
                let listed = candidates(&matching, &answers);
                assert!(
                    listed.iter().any(|(entity, legality)| *entity == *bears
                        && *legality == rules::Legality::Legal)
                );
                assert!(listed.iter().any(|(entity, legality)| *entity == *forest
                    && *legality == rules::Legality::Illegal(vec![rules::Why::NotMatching])));
                let checked = candidates(&targeted, &answers);
                assert!(
                    checked.iter().any(|(entity, legality)| *entity == *bears
                        && *legality == rules::Legality::Legal)
                );
                assert!(checked.iter().any(|(entity, legality)| *entity == *scout
                    && matches!(
                        legality,
                        rules::Legality::Illegal(reasons)
                            if reasons.contains(&rules::Why::Restriction(
                                keywords::hexproof::CANT_BE_TARGETED
                            ))
                    )));
            })
            .unwrap();
        Ok(())
    }
}
