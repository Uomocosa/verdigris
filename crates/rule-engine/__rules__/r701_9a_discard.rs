use bevy::prelude::{MessageReader, MessageWriter};

use crate::Attempt;
use crate::Did;
use crate::Resolve;
use crate::events;

/// 701.9a To discard a card, move it from its owner’s hand to that player’s graveyard.
pub fn r701_9a_discard(
    mut discards: MessageReader<Resolve<events::DiscardCard>>,
    mut graveyards: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    mut did_discard: MessageWriter<Did<events::DiscardCard>>,
) {
    for discard in discards.read() {
        graveyards.write(Attempt(events::PutIntoGraveyard {
            object: discard.card,
        }));
        did_discard.write(Did(**discard));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn mind_rot_discards_every_card_when_the_hand_holds_fewer_than_two() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([GRIZZLY_BEARS]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;

        assert!(game.moved(bears).to(GRAVEYARD).of(p2));
        assert!(!game.card(bears).is_in(HAND).of(p2));
        Ok(())
    }
}
