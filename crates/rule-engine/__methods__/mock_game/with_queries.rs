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
    use crate::players;
    use crate::testing::*;
    use crate::zones;

    use super::with_queries;

    #[test]
    fn test_usage() {
        let (mut game, ..) = MockGame::new([P1, P2]);

        with_queries::<&players::Life, &zones::Zone, ()>(&mut game, |lives, found| {
            assert!(lives.iter().all(|life| **life == 20));
            assert!(found.iter().any(|zone| *zone == zones::Zone::Battlefield));
        });
    }
}
