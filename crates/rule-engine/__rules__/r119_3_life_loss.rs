use bevy::prelude::{MessageReader, Query};

use crate::rules::helpers;
use crate::zones;

/// 119.3 If an effect causes a player to gain life or lose life, that player's life total is
/// adjusted accordingly.
pub fn r119_3_life_loss(
    mut life_lost: MessageReader<zones::LifeLost>,
    mut lives: Query<&mut zones::Life>,
) {
    for event in life_lost.read() {
        if let Ok(mut life) = lives.get_mut(event.player) {
            *life = zones::Life((**life).saturating_sub(helpers::signed_damage(event.amount)));
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    fn cast_and_resolve(game: &mut testing::MockGame, spell_name: &str, target: Option<Entity>) {
        let spell = game.find(game.p1, zones::Zone::Hand, spell_name).unwrap();
        game.act(game.p1, actions::Action::Cast(spell));
        if let Some(target) = target {
            game.act(game.p1, actions::Action::ChooseTargets(vec![target]));
        }
        game.pass_priority_all();
    }

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::sign_in_blood()])
            .mana_pool(testing::Seat::P1, mana![B, B])
            .build();
        let p2 = game.p2;
        cast_and_resolve(&mut game, "Sign in Blood", Some(p2));
        assert_eq!(game.life(p2), 18);
    }

    #[test]
    fn sign_in_blood_makes_the_target_draw_two_cards_and_lose_two_life() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::sign_in_blood()])
            .library(
                testing::Seat::P2,
                [testing::lightning_bolt(), testing::murder()],
            )
            .mana_pool(testing::Seat::P1, mana![B, B])
            .build();
        let p2 = game.p2;
        cast_and_resolve(&mut game, "Sign in Blood", Some(p2));

        assert_eq!(game.life(p2), 18);
        assert_eq!(game.life(game.p1), 20);
        assert!(game.find(p2, zones::Zone::Hand, "Lightning Bolt").is_some());
        assert!(game.find(p2, zones::Zone::Hand, "Murder").is_some());
    }

    #[test]
    fn sign_in_blood_can_target_its_controller() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::sign_in_blood()])
            .library(
                testing::Seat::P1,
                [testing::lightning_bolt(), testing::murder()],
            )
            .mana_pool(testing::Seat::P1, mana![B, B])
            .build();
        let p1 = game.p1;
        cast_and_resolve(&mut game, "Sign in Blood", Some(p1));

        assert_eq!(game.life(p1), 18);
        assert!(game.find(p1, zones::Zone::Hand, "Lightning Bolt").is_some());
        assert!(game.find(p1, zones::Zone::Hand, "Murder").is_some());
    }

    #[test]
    fn sign_in_blood_targeting_an_empty_library_loses_the_game_at_the_next_state_based_check() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::sign_in_blood()])
            .library(testing::Seat::P2, [])
            .mana_pool(testing::Seat::P1, mana![B, B])
            .build();
        let p2 = game.p2;
        cast_and_resolve(&mut game, "Sign in Blood", Some(p2));
        assert!(game.app.world().get::<zones::Lost>(p2).is_some());
    }
}
