use crate::events;
use bevy::prelude::{Commands, MessageReader, Query};

use crate::Resolve;
use crate::players;

/// 122.1. A counter is a marker placed on an object or player that modifies its characteristics
/// and/or interacts with a rule, ability, or effect. Counters are not objects and have no
/// characteristics. Notably, a counter is not a token, and a token is not a counter. Counters with
/// the same name or description are interchangeable.
pub fn r122_1_give_poison_counters(
    mut given: MessageReader<Resolve<events::GivePoisonCounters>>,
    mut commands: Commands,
    mut poisoned: Query<&mut players::PoisonCounters>,
) {
    for event in given.read() {
        if let Ok(mut counters) = poisoned.get_mut(*event.player) {
            *counters = players::PoisonCounters((**counters).saturating_add(event.amount));
        } else {
            commands.entity(*event.player).insert(players::PoisonCounters(event.amount));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::players;
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

        assert_eq!(
            game.get::<players::PoisonCounters>(p2),
            Some(players::PoisonCounters(3))
        );
        Ok(())
    }
}
