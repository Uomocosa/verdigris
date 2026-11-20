use bevy::prelude::{Commands, MessageReader, Query};

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
/// 120.3e Damage dealt to a creature by a source with neither wither nor infect causes that much
/// damage to be marked on that creature.
pub fn r120_3_r120_3a_r120_3e_damage_results(
    mut damage_dealt: MessageReader<damage::DamageDealt>,
    mut commands: Commands,
    mut lives: Query<&mut players::Life>,
    sources: Query<&cards::CardDefinition>,
    targets: Query<Option<&damage::Damage>>,
) {
    for event in damage_dealt.read() {
        if event.amount == 0 {
            continue;
        }
        if let Ok(mut life) = lives.get_mut(event.target) {
            *life =
                players::Life((**life).saturating_sub(damage::Damage::signed_amount(event.amount)));
            continue;
        }
        let marked = targets.get(event.target).ok().flatten().map_or(0, |damage| **damage);
        commands.entity(event.target).insert(damage::Damage(marked.saturating_add(event.amount)));
        let has_deathtouch = sources
            .get(event.source)
            .is_ok_and(|definition| definition.keywords.contains(&cards::Keyword::Deathtouch));
        if has_deathtouch {
            commands.entity(event.target).insert(damage::DealtByDeathtouch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn test_usage() -> TestResult {
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
        assert_eq!(game.zone_of(corpse), GRAVEYARD);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }
}
