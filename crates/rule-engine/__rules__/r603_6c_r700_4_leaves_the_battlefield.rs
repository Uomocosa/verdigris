use bevy::prelude::{Query, RemovedComponents, ResMut};

use crate::GRAVEYARD;
use crate::abilities;
use crate::cards;
use crate::stack;
use crate::zones;

/// 603.6c Leaves-the-battlefield abilities trigger when a permanent moves from the battlefield to
/// another zone, or when a phased-in permanent leaves the game because its owner leaves the game.
/// These are written as, but aren’t limited to, “When [this object] leaves the battlefield, . . .”
/// or “Whenever [something] is put into a graveyard from the battlefield, . . . .” (See also rule
/// 603.10.) An ability that attempts to do something to the card that left the battlefield checks
/// for it only in the first zone that it went to. An ability that triggers when a card is put into
/// a certain zone “from anywhere” is never treated as a leaves-the-battlefield ability, even if an
/// object is put into that zone from the battlefield.
///
/// 700.4. The term dies means “is put into a graveyard from the battlefield.”
pub fn r603_6c_r700_4_leaves_the_battlefield(
    mut left: RemovedComponents<cards::OnBattlefield>,
    mut queue: ResMut<stack::TriggerQueue>,
    cards: Query<(&cards::CardDefinition, &cards::Controller, &zones::InZone)>,
    zones: zones::ZoneLookup,
) {
    for source in left.read() {
        let Ok((definition, _, in_zone)) = cards.get(source) else {
            continue;
        };
        if !zones.contains(GRAVEYARD, **in_zone) {
            continue;
        }
        for ability in definition.triggered_abilities() {
            if !matches!(ability.event, abilities::TriggerEvent::Dies) {
                continue;
            }
            queue.push(stack::PendingTrigger {
                source,
                ability: ability.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND, LIBRARY};

    #[test]
    fn spore_crawler_draws_a_card_when_it_dies() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1.battlefield([SPORE_CRAWLER]).library([LIGHTNING_BOLT, MURDER]),
                P2.active().hand([MURDER]).mana_pool(mana![1, B, B]),
            ]);
        let [crawler] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SPORE_CRAWLER)?;
        let [murder] = game.player(p2).cards_in(HAND).exactly::<1>(MURDER)?;
        game.play([
            p2.cast(murder).targeting(crawler).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(crawler));
        assert!(game.player(p1).cards_in(LIBRARY).has(LIGHTNING_BOLT));
        game.play([p2.pass_priority(), p1.pass_priority()])?;
        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        Ok(())
    }
}
