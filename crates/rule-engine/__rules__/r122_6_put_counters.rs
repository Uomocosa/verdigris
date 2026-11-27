use crate::events;
use bevy::prelude::{Commands, MessageReader, Query};

use crate::Resolve;
use crate::counters;

/// 122.6. Some spells and abilities refer to counters being put on an object. This refers to
/// putting counters on that object while it’s on the battlefield and also to an object that’s
/// given counters as it enters the battlefield.
pub fn r122_6_put_counters(
    mut given: MessageReader<Resolve<events::PutCounters>>,
    mut commands: Commands,
    mut counted: Query<&mut counters::Counters>,
) {
    for event in given.read() {
        if let Ok(mut held) = counted.get_mut(event.recipient) {
            held.add(event.kind, event.amount);
        } else {
            let mut held = counters::Counters::default();
            held.add(event.kind, event.amount);
            commands.entity(event.recipient).insert(held);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::counters;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn caress_of_phyrexia_gives_three_poison_counters() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([CARESS_OF_PHYREXIA]).mana_pool(mana![3, B, B]), P2]);
        let [caress] = game.player(p1).cards_in(HAND).exactly::<1>(CARESS_OF_PHYREXIA)?;
        game.play([
            p1.cast(caress).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;

        let mut expected = counters::Counters::default();
        expected.add(counters::POISON, 3);
        assert_eq!(game.get::<counters::Counters>(p2), Some(expected));
        Ok(())
    }
}
