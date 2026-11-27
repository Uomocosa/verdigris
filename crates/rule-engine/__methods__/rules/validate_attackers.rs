use bevy::prelude::Entity;

use crate::Game;
use crate::actions;
use crate::cards;
use crate::events;
use crate::players;
use crate::rules;
use crate::turns;

pub fn validate_attackers(
    answers: &rules::Rules,
    game: &Game,
    player: players::PlayerId,
    attackers: &[Entity],
) -> Result<(), Vec<actions::DeclarationWhy>> {
    let mut whys: Vec<actions::DeclarationWhy> = Vec::new();
    if game.turn().step() != Some(turns::Step::DeclareAttackers)
        || game.priority_holder().is_none_or(|holder| holder.id != player)
        || game.active_player().is_none_or(|active_player| active_player.id != player)
    {
        whys.push(actions::DeclarationWhy {
            creature: None,
            why: rules::Why::WrongTiming,
        });
    }
    let is_controlled_creature = |attacker: Entity| {
        answers.entities.get(attacker).is_ok_and(|entity| {
            entity.get::<cards::Creature>().is_some()
                && entity
                    .get::<cards::Controller>()
                    .is_some_and(|controller| **controller == player)
        })
    };
    for attacker in attackers {
        if !is_controlled_creature(*attacker) {
            whys.push(actions::DeclarationWhy {
                creature: Some(*attacker),
                why: rules::Why::NotYours,
            });
        }
    }
    for attacker in attackers {
        if let rules::Legality::Illegal(reasons) = answers.legality(&events::DeclareAttackers {
            attackers: vec![*attacker],
        }) {
            for why in reasons {
                whys.push(actions::DeclarationWhy {
                    creature: Some(*attacker),
                    why,
                });
            }
        }
    }
    for candidate in answers.entities.iter() {
        if candidate.get::<cards::Creature>().is_some()
            && candidate.get::<cards::Controller>().is_some_and(|controller| **controller == player)
            && answers.required::<events::DeclareAttackers>(candidate.id())
            && answers.legal(&events::DeclareAttackers {
                attackers: vec![candidate.id()],
            })
            && !attackers.contains(&candidate.id())
        {
            whys.push(actions::DeclarationWhy {
                creature: Some(candidate.id()),
                why: rules::Why::Required,
            });
        }
    }
    if let rules::Legality::Illegal(reasons) = answers.legality(&events::DeclareAttackers {
        attackers: attackers.to_vec(),
    }) {
        for why in reasons {
            if !whys.iter().any(|existing: &actions::DeclarationWhy| existing.why == why) {
                whys.push(actions::DeclarationWhy {
                    creature: None,
                    why,
                });
            }
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

    use super::validate_attackers;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules, game: Game| {
                assert_eq!(
                    validate_attackers(&answers, &game, p1.id, &[*bears]),
                    Ok(())
                );
                assert_eq!(
                    validate_attackers(&answers, &game, p2.id, &[]),
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
