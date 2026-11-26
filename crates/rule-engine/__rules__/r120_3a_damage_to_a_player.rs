use crate::events;
use bevy::prelude::{Has, MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Resolve;
use crate::cards;
use crate::players;

/// 120.3a Damage dealt to a player by a source without infect causes that player to lose that much
/// life.
pub fn r120_3a_damage_to_a_player(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut life_lost: MessageWriter<Attempt<events::LoseLife>>,
    players: players::Players,
    infect_sources: Query<Has<cards::Infect>>,
) {
    for event in damage_dealt.read() {
        if infect_sources.get(event.source).is_ok_and(|has_infect| has_infect) {
            continue;
        }
        let affected_players = players.affected_by(&**event);
        for player in affected_players {
            life_lost.write(Attempt(events::LoseLife {
                player: player.id,
                amount: event.amount,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn damage_does_not_make_a_player_who_cant_lose_life_lose_any() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([TEST_CANT_LOSE_LIFE]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.life(p2), 20);
        Ok(())
    }
}
