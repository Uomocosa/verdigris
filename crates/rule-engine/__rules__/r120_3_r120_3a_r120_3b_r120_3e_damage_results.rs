use crate::events;
use bevy::prelude::{Commands, Has, MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Did;
use crate::Resolve;
use crate::cards;
use crate::damage;
use crate::players;

/// 120.3. Damage may have one or more of the following results, depending on whether the recipient
/// of the damage is a player or permanent, the characteristics of the damage’s source, and the
/// characteristics of the damage’s recipient (if it’s a permanent).
///
/// 120.3a Damage dealt to a player by a source without infect causes that player to lose that much
/// life.
///
/// 120.3b Damage dealt to a player by a source with infect causes that source’s controller to give
/// the player that many poison counters.
///
/// 120.3e Damage dealt to a creature by a source with neither wither nor infect causes that much
/// damage to be marked on that creature.
pub fn r120_3_r120_3a_r120_3b_r120_3e_damage_results(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut did_damage: MessageWriter<Did<events::DealDamage>>,
    mut life_lost: MessageWriter<Attempt<events::LoseLife>>,
    mut commands: Commands,
    lives: Query<&players::Life>,
    mut poisoned: Query<&mut players::PoisonCounters>,
    sources: Query<Has<cards::Deathtouch>>,
    infect_sources: Query<Has<cards::Infect>>,
    targets: Query<Option<&damage::Damage>>,
) {
    for event in damage_dealt.read() {
        if event.amount == 0 {
            continue;
        }
        if lives.get(event.recipient).is_ok() {
            if infect_sources.get(event.source).is_ok_and(|has| has) {
                if let Ok(mut counters) = poisoned.get_mut(event.recipient) {
                    *counters = players::PoisonCounters((**counters).saturating_add(event.amount));
                } else {
                    commands.entity(event.recipient).insert(players::PoisonCounters(event.amount));
                }
            } else {
                life_lost.write(Attempt(events::LoseLife {
                    player: players::PlayerId(event.recipient),
                    amount: event.amount,
                }));
            }
        } else {
            let marked = targets.get(event.recipient).ok().flatten().map_or(0, |damage| **damage);
            commands
                .entity(event.recipient)
                .insert(damage::Damage(marked.saturating_add(event.amount)));
            let has_deathtouch = sources.get(event.source).is_ok_and(|has| has);
            if has_deathtouch {
                commands.entity(event.recipient).insert(damage::DealtByDeathtouch);
            }
        }
        did_damage.write(Did(**event));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::players;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

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

    #[test]
    fn lightning_bolt_marks_damage_and_destroys_the_creature() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(1, turns::Step::PrecombatMain).players([
                P1.battlefield([WALKING_CORPSE]),
                P2.active().hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            ]);
        let [corpse] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p2.cast(bolt).targeting(corpse).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(corpse));
        assert_eq!(game.life(p1), 20);
        Ok(())
    }

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
