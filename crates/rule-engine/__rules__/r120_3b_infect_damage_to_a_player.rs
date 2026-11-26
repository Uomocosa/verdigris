use crate::events;
use bevy::prelude::{Has, MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Resolve;
use crate::cards;
use crate::players;

/// 120.3b Damage dealt to a player by a source with infect causes that source’s controller to give
/// the player that many poison counters.
pub fn r120_3b_infect_damage_to_a_player(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut poison_given: MessageWriter<Attempt<events::GivePoisonCounters>>,
    players: players::Players,
    infect_sources: Query<Has<cards::Infect>>,
) {
    for event in damage_dealt.read() {
        let Some(player) = players.as_player(event.recipient) else {
            continue;
        };
        if !infect_sources.get(event.source).is_ok_and(|has_infect| has_infect) {
            continue;
        }
        poison_given.write(Attempt(events::GivePoisonCounters {
            player,
            amount: event.amount,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::players;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn infect_damage_gives_poison_counters_instead_of_losing_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([PLAGUE_STINGER]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [stinger] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(PLAGUE_STINGER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([stinger]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 20);
        assert_eq!(
            game.get::<players::PoisonCounters>(p2),
            Some(players::PoisonCounters(1))
        );
        Ok(())
    }
}
