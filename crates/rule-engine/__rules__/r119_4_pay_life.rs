use crate::HAND;
use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::actions;
use crate::cards;
use crate::players;
use crate::stack;
use crate::zones;

/// 119.4. If a cost or effect allows a player to pay an amount of life greater than 0, the player may do so
/// only if their life total is greater than or equal to the amount of the payment. If a player pays life, the
/// payment is subtracted from their life total; in other words, the player loses that much life.
///
/// # See also
///
/// 118.3b Paying life is done by subtracting the indicated amount of life from a player’s life total. (Players
/// can always pay 0 life.)
///
/// 107.4f Phyrexian mana symbols are colored mana symbols: {W/P} is white, {U/P} is blue, {B/P} is black,
/// {R/P} is red, and {G/P} is green. A Phyrexian mana symbol represents a cost that can be paid either with one
/// mana of its color or by paying 2 life. There are also ten hybrid Phyrexian mana symbols. A hybrid Phyrexian
/// mana symbol represents a cost that can be paid with one mana of either of its component colors or by paying
/// 2 life. A hybrid Phyrexian mana symbol is both of its component colors.
pub fn r119_4_pay_life(
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &stack::CastingStage,
    )>,
    pools: Query<&cards::ManaPool>,
    mut lives: Query<&mut players::Life>,
    zones: zones::ZoneLookup,
) {
    for (spell, definition, owner, stage) in cards.iter() {
        if *stage != stack::CastingStage::AwaitingPayment {
            continue;
        }
        if definition.cost.phyrexian.is_none() {
            continue;
        }
        let Ok(pool) = pools.get(**owner) else {
            continue;
        };
        let mut affordable = *pool;
        if affordable.pay(definition.cost) {
            continue;
        }
        let Ok(mut life) = lives.get_mut(**owner) else {
            continue;
        };
        if **life < 2 {
            rejected.write(actions::Rejected::CannotPay { spell });
            if let Some(hand) = zones.find(HAND, Some(**owner)) {
                commands.entity(spell).insert(zones::InZone(hand));
            }
            commands.entity(spell).remove::<stack::CastingStage>();
            continue;
        }
        *life = players::Life((**life).saturating_sub(2));
        commands.entity(spell).remove::<stack::CastingStage>();
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn paying_two_life_casts_gut_shot_and_deals_one_damage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([GUT_SHOT]).life(20), P2]);
        let [gut_shot] = game.player(p1).cards_in(HAND).exactly::<1>(GUT_SHOT)?;

        game.play([p1.cast(gut_shot).targeting(p2).end_turn(), p2.end_turn()])?;

        assert_eq!(game.life(p1), 18);
        assert_eq!(game.life(p2), 19);
        Ok(())
    }

    #[test]
    fn a_player_with_one_life_cannot_pay_two_life_for_gut_shot() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([GUT_SHOT]).life(1), P2]);
        let [gut_shot] = game.player(p1).cards_in(HAND).exactly::<1>(GUT_SHOT)?;

        let err = game.play([p1.cast(gut_shot).targeting(p2)]).unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotPay {
                spell: gut_shot.into()
            })
        );
        assert_eq!(game.life(p1), 1);
        assert!(game.player(p1).cards_in(HAND).has(gut_shot));
        Ok(())
    }
}
