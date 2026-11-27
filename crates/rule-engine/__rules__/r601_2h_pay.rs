use crate::HAND;
use bevy::prelude::{Commands, Entity, MessageWriter, Query, With, Without};

use crate::actions;
use crate::cards;
use crate::stack;
use crate::zones;

type UntappedPermanents<'w, 's, 'controller, 'definition> = Query<
    'w,
    's,
    (
        &'controller cards::Controller,
        &'definition cards::CardDefinition,
    ),
    (With<cards::OnBattlefield>, Without<cards::Tapped>),
>;

/// 601.2h The player pays the total cost. First, they pay all costs that don’t involve random
/// elements or moving objects from the library to a public zone, in any order. Then they pay all
/// remaining costs in any order. Partial payments are not allowed. Unpayable costs can’t be paid.
pub fn r601_2h_pay(
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Controller,
        &cards::Owner,
        &stack::CastingStage,
    )>,
    mut pools: Query<&mut cards::ManaPool>,
    untapped_permanents: UntappedPermanents<'_, '_, '_, '_>,
    zones: zones::ZoneLookup,
) {
    for (spell, definition, controller, owner, stage) in cards.iter() {
        if *stage != stack::CastingStage::AwaitingPayment {
            continue;
        }
        let Ok(mut pool) = pools.get_mut(***controller) else {
            continue;
        };
        if pool.pay(definition.cost) {
            commands.entity(spell).remove::<stack::CastingStage>();
            continue;
        }
        let does_controller_have_an_untapped_permanent_with_a_mana_ability =
            untapped_permanents.iter().any(|(permanent_controller, permanent)| {
                **permanent_controller == **controller && permanent.has_mana_ability()
            });
        if !does_controller_have_an_untapped_permanent_with_a_mana_ability {
            rejected.write(actions::Rejected::CannotPay { spell });
            if let Some(hand) = zones.find(HAND, Some(**owner)) {
                commands.entity(spell).insert(zones::InZone(hand));
            }
            commands.entity(spell).remove::<stack::CastingStage>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn pays_the_cost_and_casts_the_spell() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2)])?;
        assert!(game.last_rejection().is_none());
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        Ok(())
    }

    #[test]
    fn cannot_cast_without_mana() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let err = game.play([p1.cast(bolt).targeting(p2)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotPay { spell: bolt.into() })
        );
        assert!(game.card(bolt).is_in(HAND).of(p1));
        Ok(())
    }
}
