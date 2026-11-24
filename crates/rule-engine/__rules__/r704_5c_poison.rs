use bevy::prelude::{Entity, MessageWriter, Query};

use crate::players;
use crate::rules;

/// 704.5c If a player has ten or more poison counters, that player loses the game. Ignore this rule in Two-Headed Giant games; see rule 704.6b instead.
///
/// # See also
///
/// 122.1f If a player has ten or more poison counters, that player loses the game as a state-based action. See rule 704. A player is “poisoned” if they have one or more poison counters. (See rule 810 for additional rules for Two-Headed Giant games.)
pub fn r704_5c_poison(
    mut losses: MessageWriter<rules::Would<rules::LoseGame>>,
    poisoned: Query<(Entity, &players::PoisonCounters), players::StillInTheGame>,
) {
    for (player, poison_counters) in poisoned.iter() {
        if **poison_counters >= 10 {
            losses.write(rules::Would(rules::LoseGame(players::PlayerId(player))));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn ten_poison_counters_lose_the_game() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([
                CARESS_OF_PHYREXIA,
                CARESS_OF_PHYREXIA,
                CARESS_OF_PHYREXIA,
                CARESS_OF_PHYREXIA,
            ])
            .mana_pool(mana![12, B, B, B, B, B, B, B, B]),
            P2,
        ]);
        let [caress_1, caress_2, caress_3, caress_4] =
            game.player(p1).cards_in(HAND).exactly::<4>(CARESS_OF_PHYREXIA)?;
        game.play([
            p1.cast(caress_1).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.cast(caress_2).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.cast(caress_3).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.cast(caress_4).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 8);
        assert!(game.has_lost(p2));
        Ok(())
    }
}
