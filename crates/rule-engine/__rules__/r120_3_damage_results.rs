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
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::walking_corpse()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .build();
        let source = game
            .find(game.p1, zones::Zone::Battlefield, "Walking Corpse")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source,
            target: bears,
            amount: 1,
        });
        game.app.update();
        let damage = game.app.world().get::<zones::Damage>(bears).unwrap();
        assert_eq!(**damage, 1);
    }

    #[test]
    fn r120_3a_damage_to_a_player_causes_life_loss() {
        let mut game = testing::MockGame::builder().build();
        let source = game.p1;
        let target = game.p2;
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source,
            target,
            amount: 3,
        });
        game.app.update();
        assert_eq!(game.life(target), 17);
    }
}
