use bevy::ecs::query::QueryData;
use bevy::prelude::Query;

use crate::testing;

pub fn with_queries<D: QueryData, E: QueryData, R>(
    game: &mut testing::MockGame,
    with: impl FnOnce(Query<'_, '_, D::ReadOnly>, Query<'_, '_, E::ReadOnly>) -> R,
) -> R {
    let mut first = game.app.world_mut().query::<D>();
    let mut second = game.app.world_mut().query::<E>();
    with(
        first.query(game.app.world()),
        second.query(game.app.world()),
    )
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::players;
    use crate::testing::*;
    use crate::zones;

    use super::with_queries;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let battlefield = game.zone_entity(p1, BATTLEFIELD).unwrap();

        with_queries::<&players::Life, (Entity, &zones::Zone, Option<&cards::Owner>), ()>(
            &mut game,
            |lives, zone_entities| {
                assert!(lives.iter().all(|life| **life == 20));
                assert!(zones::Zone::Battlefield.contains_card(battlefield, &zone_entities));
            },
        );
    }
}
