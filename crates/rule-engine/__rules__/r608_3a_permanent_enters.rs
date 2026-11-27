use crate::events;
use crate::{BATTLEFIELD, GRAVEYARD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, With};

use crate::Attempt;
use crate::cards;
use crate::stack;
use crate::zones;

/// 608.3a If the object that’s resolving has no targets, it becomes a permanent and enters the
/// battlefield under the control of the spell’s controller.
///
/// # See also
///
/// 608.3. If the object that’s resolving is a permanent spell, its resolution may involve several
/// steps. The instructions in rules 608.3a and b are always performed first. Then one of the steps
/// in rule 608.3c–e is performed, if appropriate.
///
/// 608.3b If the object that’s resolving has a target, it checks whether the target is still legal,
/// as described in 608.2b. If a spell with an illegal target is a bestowed Aura spell (see rule
/// 702.103e) or a mutating creature spell (see rule 702.140b), it becomes a creature spell and will
/// resolve as described in rule 608.3a. Otherwise, the spell doesn’t resolve. It is removed from
/// the stack and put into its owner’s graveyard.
pub fn r608_3a_permanent_enters(
    mut resolutions: MessageReader<stack::Resolution>,
    mut entering: MessageWriter<Attempt<events::EnterBattlefield>>,
    mut commands: Commands,
    cards: Query<(
        &cards::CardDefinition,
        &cards::Controller,
        &cards::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    battlefield: Query<Entity, With<cards::OnBattlefield>>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, controller, owner, in_zone, chosen)) = cards.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
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
        if let Some(enchant_filter) = definition.enchant() {
            let is_legal = chosen.is_some_and(|chosen| {
                let target = **chosen;
                let still_matches = cards.get(target).is_ok_and(|(definition, _, _, _, _)| {
                    enchant_filter.does_creature_match(definition)
                });
                still_matches && battlefield.contains(target)
            });
            if !is_legal {
                if let Some(graveyard) = zones.find(GRAVEYARD, Some(**owner)) {
                    commands.entity(spell).insert(zones::InZone(graveyard));
                }
                continue;
            }
            if let Some(chosen) = chosen {
                commands.entity(spell).insert(cards::Enchanting(**chosen));
            }
        }
        if zones.find(BATTLEFIELD, Some(**owner)).is_some() {
            entering.write(Attempt(events::EnterBattlefield {
                permanent: spell,
                controller: **controller,
                summoning_sick: true,
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
    fn cast_bears_enters_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        assert!(game.card(bears).is_in(BATTLEFIELD).of(p1));
        Ok(())
    }

    #[test]
    fn aura_with_an_illegal_target_goes_to_the_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([HOLY_STRENGTH]).mana_pool(mana![W]),
            P2.hand([LIGHTNING_BOLT]).mana_pool(mana![R]).battlefield([GRIZZLY_BEARS]),
        ]);
        let [strength] = game.player(p1).cards_in(HAND).exactly::<1>(HOLY_STRENGTH)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.cast(strength).targeting(bears).pass_priority(),
            p2.cast(bolt).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.moved(bears).to(GRAVEYARD).of(p2));
        assert!(!game.stack().matching(strength).is_empty());

        let [bears_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p2.arrange_graveyard([bears_buried, bolt_buried])])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.card(strength).is_in(GRAVEYARD).of(p1));
        assert!(!game.card(strength).is_in(BATTLEFIELD).of(p1));
        Ok(())
    }
}
