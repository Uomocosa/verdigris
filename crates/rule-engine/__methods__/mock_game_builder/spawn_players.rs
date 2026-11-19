use bevy::prelude::{App, Entity};

use crate::cards;
use crate::players;
use crate::testing;

pub fn spawn_players(
    builder: &testing::MockGameBuilder,
    app: &mut App,
    seats: &[testing::Seat],
) -> Vec<Entity> {
    seats
        .iter()
        .take(builder.player_count)
        .map(|seat| {
            let life = builder.lives.get(seat).copied().unwrap_or(20);
            app.world_mut()
                .spawn((
                    players::Player,
                    players::Life(life),
                    cards::ManaPool::default(),
                ))
                .id()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use crate::players;
    use crate::testing::*;

    use super::spawn_players;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let builder = MockGameBuilder::default();
        let spawned = spawn_players(&builder, &mut app, &[Seat::P1, Seat::P2]);
        assert_eq!(spawned.len(), 2);
        assert_eq!(
            app.world().get::<players::Life>(spawned[0]),
            Some(&players::Life(20))
        );
    }
}
