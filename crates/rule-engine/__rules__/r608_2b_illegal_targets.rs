use bevy::prelude::{Commands, Entity, MessageReader, Query, With};

use crate::cards;
use crate::effects;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 608.2b If the spell or ability specifies targets, it checks whether the targets are still
/// legal. ... If all its targets, for every instance of the word "target," are now illegal, the
/// spell or ability doesn't resolve. It's removed from the stack and, if it's a spell, put into
/// its owner's graveyard.
pub fn r608_2b_illegal_targets(
    mut resolutions: MessageReader<stack::Resolution>,
    mut commands: Commands,
    cards: Query<(
        &cards::CardDefinition,
        &zones::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    players: Query<Entity, With<zones::Player>>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone, chosen)) = cards.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, &zone_entities) {
            continue;
        }
        let Some(effect) = definition.effect else {
            continue;
        };
        let Some(requirement) = effect.target else {
            continue;
        };
        let is_legal = chosen.map(|chosen| **chosen).is_some_and(|target| {
            target_is_legal(target, requirement, &players, &cards, &zone_entities)
        });
        if is_legal {
            continue;
        }
        if let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(**owner), &zone_entities)
        {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

fn target_is_legal(
    target: Entity,
    requirement: effects::Target,
    players: &Query<Entity, With<zones::Player>>,
    cards: &Query<(
        &cards::CardDefinition,
        &zones::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) -> bool {
    let is_player = players.contains(target);
    let creature_matches_filter = cards.get(target).is_ok_and(|(definition, _, in_zone, _)| {
        helpers::does_creature_match_filter(definition, requirement.filter)
            && helpers::is_card_in_zone(**in_zone, zones::Zone::Battlefield, zone_entities)
    });
    match requirement.filter.kind {
        effects::FilterKind::AnyTarget => is_player || creature_matches_filter,
        effects::FilterKind::Creature => creature_matches_filter,
        effects::FilterKind::Spell
        | effects::FilterKind::CreatureSpell
        | effects::FilterKind::NoncreatureSpell
        | effects::FilterKind::InstantSpell => {
            cards.get(target).is_ok_and(|(definition, _, in_zone, _)| {
                helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, zone_entities)
                    && helpers::does_spell_match_filter(definition, requirement.filter)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
    }

    #[test]
    fn test_bolt_fizzles_when_target_gone() {
        let mut game = testing::MockGame::builder()
            .hand(
                testing::Seat::P1,
                [testing::lightning_bolt(), testing::murder()],
            )
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![1, B, B, R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let murder = game.find(game.p1, zones::Zone::Hand, "Murder").unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();

        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));
        game.act(game.p1, actions::Action::Cast(murder));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));

        game.pass_priority_all();
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(murder), zones::Zone::Graveyard);

        game.pass_priority_all();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.life(game.p2), 20);
    }

    #[test]
    fn doom_blade_fizzles_when_its_target_becomes_black() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::doom_blade()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![1, B])
            .build();
        let blade = game.find(game.p1, zones::Zone::Hand, "Doom Blade").unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();

        game.act(game.p1, actions::Action::Cast(blade));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));

        let mut definition = game
            .app
            .world()
            .get::<cards::CardDefinition>(bears)
            .cloned()
            .unwrap();
        definition.cost.black = definition.cost.black.saturating_add(1);
        game.app.world_mut().entity_mut(bears).insert(definition);

        game.pass_priority_all();
        assert_eq!(game.zone_of(bears), zones::Zone::Battlefield);
        assert_eq!(game.zone_of(blade), zones::Zone::Graveyard);
    }
}
