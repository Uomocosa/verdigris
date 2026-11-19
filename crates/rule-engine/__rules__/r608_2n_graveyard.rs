use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 608.2n As the final part of an instant or sorcery spell's resolution, the spell is put into
/// its owner's graveyard. As the final part of an ability's resolution, the ability is removed
/// from the stack and ceases to exist.
pub fn r608_2n_graveyard(
    mut resolutions: MessageReader<stack::Resolution>,
    mut commands: Commands,
    cards: Query<(&cards::CardDefinition, &cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone)) = cards.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, STACK, &zone_entities) {
            continue;
        }
        let is_instant_or_sorcery = definition.types.iter().any(|card_type| {
            matches!(
                card_type,
                cards::CardType::Instant | cards::CardType::Sorcery
            )
        });
        if !is_instant_or_sorcery {
            continue;
        }
        if let Some(graveyard) = helpers::find_zone(GRAVEYARD, Some(**owner), &zone_entities) {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt()])
            .mana_pool(Seat::P1, mana![R])
            .build();
        let bolt = game.find(game.p1, HAND, "Lightning Bolt").unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert!(game.find(game.p1, GRAVEYARD, "Lightning Bolt").is_some());
    }
}
