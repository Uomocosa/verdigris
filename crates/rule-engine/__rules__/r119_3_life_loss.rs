use bevy::prelude::{MessageReader, Query};

use crate::damage;
use crate::players;
use crate::rules;

/// 119.3. If an effect causes a player to gain life or lose life, that player’s life total is
/// adjusted accordingly.
pub fn r119_3_life_loss(
    mut life_lost: MessageReader<rules::Proceeds<rules::LoseLife>>,
    mut life_gained: MessageReader<rules::Proceeds<rules::GainLife>>,
    mut lives: Query<&mut players::Life>,
) {
    for event in life_lost.read() {
        if let Ok(mut life) = lives.get_mut(*event.player) {
            *life =
                players::Life((**life).saturating_sub(damage::Damage::signed_amount(event.amount)));
        }
    }
    for event in life_gained.read() {
        if let Ok(mut life) = lives.get_mut(*event.player) {
            *life =
                players::Life((**life).saturating_add(damage::Damage::signed_amount(event.amount)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn sign_in_blood_makes_the_target_draw_two_cards_and_lose_two_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([SIGN_IN_BLOOD]).mana_pool(mana![B, B]),
            P2.library([LIGHTNING_BOLT, MURDER]),
        ]);
        let [sign_in_blood] = game.player(p1).cards_in(HAND).exactly::<1>(SIGN_IN_BLOOD)?;
        game.play([
            p1.cast(sign_in_blood).targeting(p2).end_turn(),
            p2.end_turn(),
        ])?;

        assert_eq!(game.life(p2), 18);
        assert_eq!(game.life(p1), 20);
        assert!(game.player(p2).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p2).cards_in(HAND).has(MURDER));
        Ok(())
    }

    #[test]
    fn sign_in_blood_can_target_its_controller() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([SIGN_IN_BLOOD]).library([LIGHTNING_BOLT, MURDER]).mana_pool(mana![B, B]),
            P2,
        ]);
        let [sign_in_blood] = game.player(p1).cards_in(HAND).exactly::<1>(SIGN_IN_BLOOD)?;
        game.play([
            p1.cast(sign_in_blood).targeting(p1).end_turn(),
            p2.end_turn(),
        ])?;

        assert_eq!(game.life(p1), 18);
        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p1).cards_in(HAND).has(MURDER));
        Ok(())
    }

    #[test]
    fn sign_in_blood_targeting_an_empty_library_loses_the_game_at_the_next_state_based_check()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([SIGN_IN_BLOOD]).mana_pool(mana![B, B]),
            P2.empty_library(),
        ]);
        let [sign_in_blood] = game.player(p1).cards_in(HAND).exactly::<1>(SIGN_IN_BLOOD)?;
        game.play([
            p1.cast(sign_in_blood).targeting(p2).end_turn(),
            p2.end_turn(),
        ])?;
        assert!(game.has_lost(p2));
        Ok(())
    }
}
