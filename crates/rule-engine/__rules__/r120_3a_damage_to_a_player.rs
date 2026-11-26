use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

use crate::Attempt;
use crate::LOSES_LIFE;
use crate::May;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;

/// 120.3a Damage dealt to a player by a source without infect causes that player to lose that much
/// life.
///
/// # See also
///
/// 702.90b Damage dealt to a player by a source with infect doesn’t cause that player to lose life.
/// Rather, it causes that source’s controller to give the player that many poison counters. See
/// rule 120.3.
pub fn r120_3a_damage_to_a_player(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut life_lost: MessageWriter<Attempt<events::LoseLife>>,
    mut commands: Commands,
    mut asks: MessageWriter<May<events::DealDamage>>,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    players: players::Players,
    answers: rules::Rules,
) {
    for event in damage_dealt.read() {
        let affected_players = players.affected_by(&**event);
        if affected_players.is_empty() {
            continue;
        }
        match answers.replaced(LOSES_LIFE, &**event) {
            Some(then) => then.announce(
                &**event,
                &answers,
                &mut commands,
                &mut asks,
                &mut state_based_actions_performed,
            ),
            None => {
                for player in affected_players {
                    life_lost.write(Attempt(events::LoseLife {
                        player: player.id,
                        amount: event.amount,
                    }));
                }
            }
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
