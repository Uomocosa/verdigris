use bevy::prelude::{Entity, EntityRef};

use crate::Game;
use crate::actions;
use crate::cards;
use crate::players;
use crate::rules;
use crate::turns;

pub fn legal_block_previews(
    answers: &rules::Rules,
    game: &Game,
    player: players::PlayerId,
) -> Vec<(actions::BlockAssignment, rules::Legality)> {
    if game.active_player().is_some_and(|active_player| active_player.id == player)
        || game.turn().step() != Some(turns::Step::DeclareBlockers)
    {
        return Vec::new();
    }
    let attacking: Vec<Entity> = answers
        .entities
        .iter()
        .filter(EntityRef::contains::<cards::Attacking>)
        .map(|entity| entity.id())
        .collect();
    let mut blocks: Vec<(actions::BlockAssignment, rules::Legality)> = Vec::new();
    for candidate in answers.entities.iter() {
        if candidate.get::<cards::Creature>().is_some()
            && candidate.get::<cards::Controller>().is_some_and(|controller| **controller == player)
        {
            for attacker in &attacking {
                let assignment = actions::BlockAssignment {
                    blocker: candidate.id(),
                    attacker: *attacker,
                };
                let legality = match answers.validate_blockers(game, player, &[assignment]) {
                    Ok(()) => rules::Legality::Legal,
                    Err(whys) => rules::Legality::Illegal(
                        whys.into_iter()
                            .filter(|why| {
                                why.creature == Some(assignment.blocker)
                                    || why.creature == Some(assignment.attacker)
                            })
                            .map(|why| why.why)
                            .collect(),
                    ),
                };
                blocks.push((assignment, legality));
            }
        }
    }
    blocks
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

    use super::legal_block_previews;

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
                    legal_block_previews(&answers, &game, p2.id),
                    vec![(
                        actions::BlockAssignment {
                            blocker: *bears_2,
                            attacker: *bears_1,
                        },
                        rules::Legality::Legal,
                    )]
                );
                assert_eq!(legal_block_previews(&answers, &game, p1.id), Vec::new());
            })
            .unwrap();
        Ok(())
    }
}
