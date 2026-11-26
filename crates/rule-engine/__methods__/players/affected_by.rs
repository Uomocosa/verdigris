use crate::players;
use crate::rules::traits::event::Event;

pub fn affected_by<'w, 's>(
    players: &players::Players<'w, 's>,
    event: &impl Event,
) -> Vec<players::PlayerView<'w, 's>> {
    let affected = event.affected();
    players
        .in_turn_order()
        .into_iter()
        .filter(|player| players.is_still_in_the_game(*player))
        .filter(|player| affected.contains(&**player))
        .map(|id| (*players).view(id))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::testing::*;

    use super::affected_by;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                let discard = events::Discard {
                    player: p3.id,
                    amount: 1,
                };
                assert_eq!(
                    affected_by(&all, &discard)
                        .into_iter()
                        .map(|player| player.id)
                        .collect::<Vec<_>>(),
                    [p3.id]
                );
                let damage = events::DealDamage {
                    source: Entity::PLACEHOLDER,
                    recipient: *p2,
                    amount: 1,
                    controller: p1.id,
                };
                assert!(affected_by(&all, &damage).is_empty());
            })
            .unwrap();
        Ok(())
    }
}
