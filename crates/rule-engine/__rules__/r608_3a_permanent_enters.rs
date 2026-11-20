use crate::{BATTLEFIELD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 608.3. If the object that’s resolving is a permanent spell, its resolution may involve
/// several steps. The instructions in rules 608.3a and b are always performed first. Then one
/// of the steps in rule 608.3c–e is performed, if appropriate.
///
/// 608.3a If the object that’s resolving has no targets, it becomes a permanent and enters the
/// battlefield under the control of the spell’s controller.
pub fn r608_3a_permanent_enters(
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
        let is_permanent = !definition.types.iter().any(|card_type| {
            matches!(
                card_type,
                cards::CardType::Instant | cards::CardType::Sorcery
            )
        });
        if !is_permanent {
            continue;
        }
        if let Some(battlefield) = helpers::find_zone(BATTLEFIELD, Some(**owner), &zone_entities) {
            commands.entity(spell).insert(zones::InZone(battlefield));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn test_usage() -> Result<(), TestError> {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        Ok(())
    }

    #[test]
    fn r608_3a_cast_bears_enters_the_battlefield() -> Result<(), TestError> {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        Ok(())
    }
}
