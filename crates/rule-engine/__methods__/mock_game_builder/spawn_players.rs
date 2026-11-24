use bevy::prelude::App;

use crate::players;
use crate::testing;

pub fn spawn_players(app: &mut App, setups: &[testing::PlayerSetup]) -> Vec<players::PlayerId> {
    setups
        .iter()
        .map(|setup| {
            players::PlayerId(
                app.world_mut()
                    .spawn((
                        players::Player,
                        players::Life(setup.life),
                        players::LandsPlayedThisTurn::NONE,
                        setup.mana_pool,
                    ))
                    .id(),
            )
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
        let spawned = spawn_players(&mut app, &[P1.life(7), P2]);
        assert_eq!(spawned.len(), 2);
        assert_eq!(
            app.world().get::<players::Life>(*spawned[0]),
            Some(&players::Life(7))
        );
        assert_eq!(
            app.world().get::<players::Life>(*spawned[1]),
            Some(&players::Life(20))
        );
    }
}
