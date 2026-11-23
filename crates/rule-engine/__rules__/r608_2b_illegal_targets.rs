use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, Query, With};

use crate::cards;
use crate::effects;
use crate::players;
use crate::stack;
use crate::zones;

/// 608.2b If the spell or ability specifies targets, it checks whether the targets are still legal.
/// A target that’s no longer in the zone it was in when it was targeted is illegal. Other changes
/// to the game state may cause a target to no longer be legal; for example, its characteristics may
/// have changed or an effect may have changed the text of the spell. If the source of an ability
/// has left the zone it was in, its last known information is used during this process. If all its
/// targets, for every instance of the word “target,” are now illegal, the spell or ability doesn’t
/// resolve. It’s removed from the stack and, if it’s a spell, put into its owner’s graveyard.
/// Otherwise, the spell or ability will resolve normally. Illegal targets, if any, won’t be
/// affected by parts of a resolving spell’s effect for which they’re illegal. Other parts of the
/// effect for which those targets are not illegal may still affect them. If the spell or ability
/// creates any continuous effects that affect game rules (see rule 613.11), those effects don’t
/// apply to illegal targets. If part of the effect requires information about an illegal target, it
/// fails to determine any such information. Any part of the effect that requires that information
/// won’t happen.
pub fn r608_2b_illegal_targets(
    mut resolutions: MessageReader<stack::Resolution>,
    mut commands: Commands,
    cards: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    players: Query<Entity, With<players::Player>>,
    battlefield: Query<Entity, With<cards::OnBattlefield>>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone, chosen)) = cards.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        let Some(effect) = definition.spell_ability() else {
            continue;
        };
        let Some(requirement) = effect.target else {
            continue;
        };
        let is_legal = chosen.map(|chosen| **chosen).is_some_and(|target| {
            target_is_legal(target, requirement, &players, &cards, &battlefield, &zones)
        });
        if is_legal {
            continue;
        }
        if let Some(graveyard) = zones.find(GRAVEYARD, Some(**owner)) {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

fn target_is_legal(
    target: Entity,
    requirement: effects::Target,
    players: &Query<Entity, With<players::Player>>,
    cards: &Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    battlefield: &Query<Entity, With<cards::OnBattlefield>>,
    zones: &zones::ZoneLookup,
) -> bool {
    let is_player = players.contains(target);
    let creature_matches_filter = cards
        .get(target)
        .is_ok_and(|(definition, _, _, _)| requirement.filter.does_creature_match(definition))
        && battlefield.contains(target);
    let enchantment_matches_filter = cards
        .get(target)
        .is_ok_and(|(definition, _, _, _)| requirement.filter.does_enchantment_match(definition))
        && battlefield.contains(target);
    match requirement.filter.kind {
        effects::FilterKind::AnyTarget => is_player || creature_matches_filter,
        effects::FilterKind::Player => is_player,
        effects::FilterKind::Creature => creature_matches_filter,
        effects::FilterKind::Enchantment => enchantment_matches_filter,
        effects::FilterKind::Spell
        | effects::FilterKind::CreatureSpell
        | effects::FilterKind::NoncreatureSpell
        | effects::FilterKind::InstantSpell => {
            cards.get(target).is_ok_and(|(definition, _, in_zone, _)| {
                zones.contains(STACK, **in_zone) && requirement.filter.does_spell_match(definition)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn spell_with_legal_target_resolves_to_graveyard() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        Ok(())
    }

    #[test]
    fn bolt_fizzles_when_target_gone() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, MURDER]).mana_pool(mana![1, B, B, R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.cast(bolt).targeting(bears),
            p1.cast(murder).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(murder));
        assert!(game.stack().has(bolt));

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        assert_eq!(game.life(p2), 20);
        Ok(())
    }

    #[test]
    fn doom_blade_fizzles_when_its_target_becomes_black() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.cast(blade).targeting(bears)])?;

        let mut definition =
            game.app.world().get::<cards::CardDefinition>(bears.into()).cloned().unwrap();
        definition.cost.black = definition.cost.black.saturating_add(1);
        game.app.world_mut().entity_mut(Entity::from(bears)).insert(definition);

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(blade));
        Ok(())
    }
}
