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
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::STACK;
    use crate::cards;
    use crate::testing::*;
    use crate::zones;

    use super::with_query;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let battlefield = game.zone_entity(p1, BATTLEFIELD).unwrap();
        assert_eq!(game.zone_entity(p1, STACK), Some(game.stack));

        with_query::<(Entity, &zones::Zone, Option<&cards::Owner>), ()>(
            &mut game,
            |zone_entities| {
                assert!(zones::Zone::Battlefield.contains_card(battlefield, &zone_entities));
                assert!(!zones::Zone::Hand.contains_card(battlefield, &zone_entities));
            },
        );
    }
}
