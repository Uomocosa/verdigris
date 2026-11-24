use crate::HAND;
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query};

use crate::actions;
use crate::cards;
use crate::stack;
use crate::zones;

/// 601.2g If the total cost includes a mana payment, the player then has a chance to activate
/// mana abilities (see rule 605, “Mana Abilities”). Mana abilities must be activated before
/// costs are paid.
pub fn r601_2g_mana_abilities(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    casting: Query<(
        Entity,
        &stack::CastingStage,
        &cards::Owner,
        &cards::CardDefinition,
    )>,
    mut pools: Query<&mut cards::ManaPool>,
    zones: zones::ZoneLookup,
) {
    for action in actions.read() {
        let is_mana_activation =
            matches!(&action.action, actions::Action::ActivateManaAbility { .. });
        let is_pass = matches!(&action.action, actions::Action::PassPriority);
        if !is_mana_activation && !is_pass {
            continue;
        }
        for (spell, stage, owner, definition) in casting.iter() {
            if *stage != stack::CastingStage::AwaitingPayment || **owner != action.player {
                continue;
            }
            if is_mana_activation {
                let Ok(mut pool) = pools.get_mut(*action.player) else {
                    continue;
                };
                if pool.pay(definition.cost) {
                    commands.entity(spell).remove::<stack::CastingStage>();
                }
            } else {
                rejected.write(actions::Rejected::CannotPay { spell });
                if let Some(hand) = zones.find(HAND, Some(**owner)) {
                    commands.entity(spell).insert(zones::InZone(hand));
                }
                commands.entity(spell).remove::<stack::CastingStage>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn a_land_can_be_tapped_for_mana_while_casting_a_creature() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([
            P1.battlefield([FOREST]).hand([GRIZZLY_BEARS]).mana_pool(mana![1]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;

        game.play([p1.cast(bears).activate_ability(forest.ability())])?;

        assert!(game.stack().has(bears));
        assert!(game.is_tapped(forest));
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        Ok(())
    }

    #[test]
    fn passing_priority_without_paying_abandons_the_cast() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([
            P1.battlefield([FOREST]).hand([GRIZZLY_BEARS]).mana_pool(mana![1]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.cast(bears)])?;
        let err = game.play([p1.pass_priority()]).unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotPay {
                spell: bears.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(bears));
        assert!(!game.stack().has(bears));
        assert_eq!(game.mana_pool(p1), mana![1]);
        Ok(())
    }
}
