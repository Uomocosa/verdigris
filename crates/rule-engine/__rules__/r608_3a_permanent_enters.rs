use crate::{BATTLEFIELD, STACK};
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res};

use crate::cards;
use crate::stack;
use crate::turns;
use crate::zones;

/// 608.3a If the object that’s resolving has no targets, it becomes a permanent and enters the
/// battlefield under the control of the spell’s controller.
///
/// # See also
///
/// 608.3. If the object that’s resolving is a permanent spell, its resolution may involve several
/// steps. The instructions in rules 608.3a and b are always performed first. Then one of the steps
/// in rule 608.3c–e is performed, if appropriate.
pub fn r608_3a_permanent_enters(
    mut resolutions: MessageReader<stack::Resolution>,
    mut entered: MessageWriter<cards::Entered>,
    mut commands: Commands,
    cards: Query<(&cards::CardDefinition, &cards::Owner, &zones::InZone)>,
    zones: zones::ZoneLookup,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone)) = cards.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
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
        if let Some(battlefield) = zones.find(BATTLEFIELD, Some(**owner)) {
            commands.entity(spell).insert((
                zones::InZone(battlefield),
                cards::EnteredOnTurn(turn.number),
            ));
            entered.write(cards::Entered(spell));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn cast_bears_enters_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(bears));
        Ok(())
    }
}
