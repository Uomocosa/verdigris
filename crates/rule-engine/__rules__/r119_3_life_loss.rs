use bevy::prelude::{MessageReader, Query};

use crate::players;
use crate::rules::helpers;

/// 119.3 If an effect causes a player to gain life or lose life, that player's life total is
/// adjusted accordingly.
pub fn r119_3_life_loss(
    mut life_lost: MessageReader<players::LifeLost>,
    mut lives: Query<&mut players::Life>,
) {
    for event in life_lost.read() {
        if let Ok(mut life) = lives.get_mut(event.player) {
            *life = players::Life((**life).saturating_sub(helpers::signed_damage(event.amount)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::players;
    use crate::testing::traits::Caster;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([sign_in_blood()]).mana_pool(mana![B, B]), P2]);
        let sign_in_blood = game.player(p1).cards(HAND, SIGN_IN_BLOOD).only().unwrap();
        game.play(p1.cast(sign_in_blood).targeting(p2).resolve())
            .unwrap();
        assert_eq!(game.life(p2), 18);
    }

    #[test]
    fn sign_in_blood_makes_the_target_draw_two_cards_and_lose_two_life() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([sign_in_blood()]).mana_pool(mana![B, B]),
            P2.library([lightning_bolt(), murder()]),
        ]);
        let sign_in_blood = game.player(p1).cards(HAND, SIGN_IN_BLOOD).only().unwrap();
        game.play(p1.cast(sign_in_blood).targeting(p2).resolve())
            .unwrap();

        assert_eq!(game.life(p2), 18);
        assert_eq!(game.life(p1), 20);
        assert!(game.find(p2, HAND, "Lightning Bolt").is_some());
        assert!(game.find(p2, HAND, "Murder").is_some());
    }

    #[test]
    fn sign_in_blood_can_target_its_controller() {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([sign_in_blood()])
                .library([lightning_bolt(), murder()])
                .mana_pool(mana![B, B]),
            P2,
        ]);
        let sign_in_blood = game.player(p1).cards(HAND, SIGN_IN_BLOOD).only().unwrap();
        game.play(p1.cast(sign_in_blood).targeting(p1).resolve())
            .unwrap();

        assert_eq!(game.life(p1), 18);
        assert!(game.find(p1, HAND, "Lightning Bolt").is_some());
        assert!(game.find(p1, HAND, "Murder").is_some());
    }

    #[test]
    fn sign_in_blood_targeting_an_empty_library_loses_the_game_at_the_next_state_based_check() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([sign_in_blood()]).mana_pool(mana![B, B]),
            P2.library([]),
        ]);
        let sign_in_blood = game.player(p1).cards(HAND, SIGN_IN_BLOOD).only().unwrap();
        game.play(p1.cast(sign_in_blood).targeting(p2).resolve())
            .unwrap();
        assert!(game.app.world().get::<players::Lost>(p2).is_some());
    }
}
