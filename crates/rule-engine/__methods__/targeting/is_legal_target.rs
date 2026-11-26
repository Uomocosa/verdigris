use bevy::prelude::Entity;
use bevy::prelude::EntityRef;
use std::collections::BTreeSet;

use crate::STACK;
use crate::cards;
use crate::effects;
use crate::events;
use crate::players;
use crate::stack;
use crate::zones;

pub fn is_legal_target(
    targeting: &stack::Targeting,
    spell: Entity,
    requirement: effects::Target,
    controller: players::PlayerId,
    target: Entity,
) -> bool {
    if target == spell {
        return false;
    }
    let filter = requirement.filter;
    let all_players: Vec<players::PlayerId> =
        targeting.players.iter().map(players::PlayerId).collect();
    let candidate = targeting.answers.entities.get(target).ok();
    let definition = candidate.as_ref().and_then(EntityRef::get::<cards::CardDefinition>);
    let candidate_controller = candidate.as_ref().and_then(|candidate| {
        candidate
            .get::<cards::Controller>()
            .map(|controller| **controller)
            .or_else(|| candidate.get::<cards::Owner>().map(|owner| **owner))
    });
    let in_zone = candidate.as_ref().and_then(EntityRef::get::<zones::InZone>);
    let on_battlefield =
        candidate.as_ref().is_some_and(|entity| entity.get::<cards::OnBattlefield>().is_some());
    if let Some(allowed) = filter
        .controlled_by
        .map(|player| player.resolve(controller, &all_players).into_iter().collect::<BTreeSet<_>>())
        && let Some(candidate_controller) = candidate_controller
        && !allowed.contains(&candidate_controller)
    {
        return false;
    }
    let is_player = targeting.players.contains(target);
    let creature_matches_filter = definition
        .is_some_and(|definition| filter.does_creature_match(definition))
        && on_battlefield;
    let enchantment_matches_filter = definition
        .is_some_and(|definition| filter.does_enchantment_match(definition))
        && on_battlefield;
    let matches_filter = match filter.kind {
        effects::FilterKind::AnyTarget => is_player || creature_matches_filter,
        effects::FilterKind::Player => is_player,
        effects::FilterKind::Creature => creature_matches_filter,
        effects::FilterKind::Enchantment => enchantment_matches_filter,
        effects::FilterKind::Spell
        | effects::FilterKind::CreatureSpell
        | effects::FilterKind::NoncreatureSpell
        | effects::FilterKind::InstantSpell => {
            definition.is_some_and(|definition| filter.does_spell_match(definition))
                && in_zone.is_some_and(|in_zone| targeting.answers.zones.contains(STACK, **in_zone))
        }
    };
    matches_filter
        && targeting.answers.legal(&events::BecomeTarget {
            target,
            spell,
            player: controller,
        })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{ANY_TARGET, BATTLEFIELD, ENCHANTMENT, HAND};

    use super::is_legal_target;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |targeting: stack::Targeting| {
                let requirement = effects::target(1, ANY_TARGET);
                assert!(is_legal_target(
                    &targeting,
                    *bolt,
                    requirement,
                    p1.id,
                    Entity::from(bears)
                ));
                assert!(!is_legal_target(
                    &targeting,
                    Entity::from(bears),
                    requirement,
                    p1.id,
                    Entity::from(bears)
                ));
                let enchantment = effects::target(1, ENCHANTMENT);
                assert!(!is_legal_target(
                    &targeting,
                    *bolt,
                    enchantment,
                    p1.id,
                    Entity::from(bears)
                ));
            })
            .unwrap();
        Ok(())
    }
}
