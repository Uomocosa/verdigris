use bevy::prelude::{MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::RESOLUTION_CAUSE;
use crate::STACK;
use crate::cards;
use crate::events;
use crate::rules;
use crate::stack;
use crate::zones;

/// 608.3e If a permanent spell resolves but its controller can’t put it onto the battlefield, that
/// player puts it into its owner’s graveyard.
pub fn r608_3e_cannot_enter(
    mut resolutions: MessageReader<stack::Resolution>,
    mut graveyards: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    cards: Query<(&cards::CardDefinition, &cards::Controller, &zones::InZone)>,
    answers: rules::Rules,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, _, in_zone)) = cards.get(spell) else {
            continue;
        };
        if !answers.zones.contains(STACK, **in_zone) {
            continue;
        }
        let is_permanent = !definition.type_line.types.iter().any(|card_type| {
            matches!(
                card_type,
                cards::CardType::Instant | cards::CardType::Sorcery
            )
        });
        if !is_permanent {
            continue;
        }
        if !answers.legal(&events::EnterTheBattlefield { permanent: spell }) {
            graveyards.write(Attempt(events::PutIntoGraveyard {
                object: spell,
                cause: RESOLUTION_CAUSE,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn a_permanent_spell_that_cannot_enter_is_put_into_its_owners_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GRIZZLY_BEARS]).battlefield([TEST_NO_TOKENS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        assert!(!game.card(bears).is_in(BATTLEFIELD).of(p1));
        assert!(!game.card(bears).is_on_the_stack());
        Ok(())
    }
}
