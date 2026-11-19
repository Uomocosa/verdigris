use bevy::prelude::{Commands, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::zones;

/// 120.3 Damage may have one or more of the following results, depending on whether the recipient
/// of the damage is a player or permanent, the characteristics of the damage's source, and the
/// characteristics of the damage's recipient (if it's a permanent).
/// 120.3a Damage dealt to a player by a source without infect causes that player to lose that much
/// life.
/// 120.3e Damage dealt to a creature by a source with neither wither nor infect causes that much
/// damage to be marked on that creature.
pub fn r120_3_damage_results(
    mut damage_dealt: MessageReader<zones::DamageDealt>,
    mut commands: Commands,
    mut lives: Query<&mut zones::Life>,
    sources: Query<&cards::CardDefinition>,
    targets: Query<Option<&zones::Damage>>,
) {
    for event in damage_dealt.read() {
        if event.amount == 0 {
            continue;
        }
        if let Ok(mut life) = lives.get_mut(event.target) {
            *life = zones::Life((**life).saturating_sub(helpers::signed_damage(event.amount)));
            continue;
        }
        let marked = targets
            .get(event.target)
            .ok()
            .flatten()
            .map_or(0, |damage| **damage);
        commands
            .entity(event.target)
            .insert(zones::Damage(marked.saturating_add(event.amount)));
        let has_deathtouch = sources
            .get(event.source)
            .is_ok_and(|definition| definition.keywords.contains(&cards::Keyword::Deathtouch));
        if has_deathtouch {
            commands
                .entity(event.target)
                .insert(zones::DealtByDeathtouch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .turn(1, Seat::P2, turns::Step::PrecombatMain)
            .battlefield(Seat::P1, [walking_corpse()])
            .hand(Seat::P2, [lightning_bolt()])
            .mana_pool(Seat::P2, mana![R])
            .build();
        let corpse = game
            .find(game.p1, zones::Zone::Battlefield, "Walking Corpse")
            .unwrap();
        let bolt = game
            .find(game.p2, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let p1 = game.p1;
        game.act(game.p2, actions::Action::Cast(bolt));
        game.act(game.p2, actions::Action::ChooseTargets(vec![corpse]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(corpse), zones::Zone::Graveyard);
        assert_eq!(game.life(p1), 20);
    }
}
