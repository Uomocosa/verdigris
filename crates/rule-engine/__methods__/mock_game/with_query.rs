use bevy::ecs::query::QueryData;
use bevy::prelude::Query;

use crate::testing;

pub fn with_query<D: QueryData, R>(
    game: &mut testing::MockGame,
    with: impl FnOnce(Query<'_, '_, D::ReadOnly>) -> R,
) -> R {
    let mut states = game.app.world_mut().query::<D>();
    with(states.query(game.app.world()))
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::zones;

    use super::with_query;

    #[test]
    fn test_usage() {
        let (mut game, ..) = MockGame::new([P1, P2]);

        with_query::<&zones::Zone, ()>(&mut game, |found| {
            assert!(found.iter().any(|zone| *zone == zones::Zone::Battlefield));
            assert!(found.iter().any(|zone| *zone == zones::Zone::Stack));
        });
    }
}
