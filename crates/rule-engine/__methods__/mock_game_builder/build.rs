use crate::testing;

const SEATS: [testing::Seat; 8] = [
    testing::Seat::P1,
    testing::Seat::P2,
    testing::Seat::P3,
    testing::Seat::P4,
    testing::Seat::P5,
    testing::Seat::P6,
    testing::Seat::P7,
    testing::Seat::P8,
];

pub fn build(builder: &testing::MockGameBuilder) -> testing::MockGame {
    let players: Vec<testing::PlayerSetup> = SEATS
        .iter()
        .take(builder.player_count)
        .enumerate()
        .map(|(index, seat)| testing::PlayerSetup {
            seat: index.saturating_add(1),
            hand: builder.hands.get(seat).cloned().unwrap_or_default(),
            battlefield: builder.battlefields.get(seat).cloned().unwrap_or_default(),
            library: builder.libraries.get(seat).cloned(),
            mana_pool: builder.mana_pools.get(seat).copied().unwrap_or_default(),
            life: builder.lives.get(seat).copied().unwrap_or(20),
            active: *seat == builder.active,
        })
        .collect();
    let (game, _) = builder.spawn_game(&players);
    game
}

#[cfg(test)]
mod tests {
    use crate::LIBRARY;
    use crate::testing::*;

    use super::build;

    #[test]
    fn test_usage() {
        let game = build(&MockGameBuilder::default());
        assert_eq!(game.life(game.p1), 20);
        assert!(game.find(game.p1, LIBRARY, "Grizzly Bears").is_some());
    }
}
