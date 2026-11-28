use bevy::prelude::Entity;

use crate::Game;
use crate::actions;
use crate::cards;
use crate::events;
use crate::players;
use crate::rules;
use crate::turns;

pub fn validate_blockers(
    answers: &rules::Rules,
    game: &Game,
    player: players::PlayerId,
    assignments: &[actions::BlockAssignment],
) -> Result<(), Vec<actions::DeclarationWhy>> {
    let mut whys: Vec<actions::DeclarationWhy> = Vec::new();
    if game.turn().step() != Some(turns::Step::DeclareBlockers)
        || game.priority_holder().is_none_or(|holder| holder.id != player)
        || game.active_player().is_some_and(|active| active.id == player)
    {
        whys.push(actions::DeclarationWhy {
            creature: None,
            why: rules::Why::WrongTiming,
        });
    }
    let is_controlled_creature = |blocker: Entity| {
        answers.entities.get(blocker).is_ok_and(|entity| {
            entity.get::<cards::Creature>().is_some()
                && entity
                    .get::<cards::Controller>()
                    .is_some_and(|controller| **controller == player)
        })
    };
    let is_attacking = |attacker: Entity| {
        answers.entities.get(attacker).is_ok_and(|entity| entity.contains::<cards::Attacking>())
    };
    for assignment in assignments {
        if !is_attacking(assignment.attacker) {
            let why = actions::DeclarationWhy {
                creature: Some(assignment.attacker),
                why: rules::Why::NotAttacking,
            };
            if !whys.contains(&why) {
                whys.push(why);
            }
        }
        if !is_controlled_creature(assignment.blocker) {
            let why = actions::DeclarationWhy {
                creature: Some(assignment.blocker),
                why: rules::Why::NotYours,
            };
            if !whys.contains(&why) {
                whys.push(why);
            }
        }
        if let rules::Legality::Illegal(reasons) = answers.legality(&events::DeclareBlocker {
            blocker: assignment.blocker,
            attacker: assignment.attacker,
        }) {
            for why in reasons {
                let why = actions::DeclarationWhy {
                    creature: Some(assignment.blocker),
                    why,
                };
                if !whys.contains(&why) {
                    whys.push(why);
                }
            }
        }
    }
    for assignment in assignments {
        let count = assignments.iter().filter(|other| other.blocker == assignment.blocker).count();
        if answers.value::<rules::BlocksPerCreature>(assignment.blocker).excess(count) > 0 {
            let why = actions::DeclarationWhy {
                creature: Some(assignment.blocker),
                why: rules::Why::BlocksMoreThanOne,
            };
            if !whys.contains(&why) {
                whys.push(why);
            }
        }
    }
    for candidate in answers.entities.iter() {
        if candidate.get::<cards::Creature>().is_some()
            && candidate.get::<cards::Controller>().is_some_and(|controller| **controller == player)
            && answers.required::<events::DeclareBlocker>(candidate.id())
            && !assignments.iter().any(|assignment| assignment.blocker == candidate.id())
            && answers.entities.iter().any(|attacker| {
                is_attacking(attacker.id())
                    && answers.legal::<events::DeclareBlocker>(&events::DeclareBlocker {
                        blocker: candidate.id(),
                        attacker: attacker.id(),
                    })
            })
        {
            whys.push(actions::DeclarationWhy {
                creature: Some(candidate.id()),
                why: rules::Why::Required,
            });
        }
    }
    if whys.is_empty() { Ok(()) } else { Err(whys) }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::Game;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    use super::validate_blockers;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
        ])?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules, game: Game| {
                assert_eq!(
                    validate_blockers(
                        &answers,
                        &game,
                        p2.id,
                        &[actions::BlockAssignment {
                            blocker: *bears_2,
                            attacker: *bears_1,
                        }]
                    ),
                    Ok(())
                );
                assert_eq!(
                    validate_blockers(&answers, &game, p1.id, &[]),
                    Err(vec![actions::DeclarationWhy {
                        creature: None,
                        why: rules::Why::WrongTiming,
                    }])
                );
            })
            .unwrap();
        Ok(())
    }
}
