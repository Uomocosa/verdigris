use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, ResMut, With};

use crate::actions;
use crate::cards;
use crate::effects;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 601.2c The player announces their choice of an appropriate object or player for each target
/// the spell requires. ... The chosen objects and/or players each become a target of that spell.
pub fn r601_2c_targets(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut commands: Commands,
    players: Query<Entity, With<zones::Player>>,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &zones::Owner,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for action in actions.read() {
        match &action.action {
            actions::Action::Cast(spell) => {
                let spell = *spell;
                let Ok((_, definition, owner, in_zone)) = cards.get(spell) else {
                    continue;
                };
                if !helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, &zone_entities) {
                    continue;
                }
                let controller = **owner;
                let Some(effect) = definition.effect else {
                    commands
                        .entity(spell)
                        .insert(stack::CastingStage::AwaitingPayment);
                    continue;
                };
                let Some(target) = effect.target else {
                    commands
                        .entity(spell)
                        .insert(stack::CastingStage::AwaitingPayment);
                    continue;
                };
                let legal =
                    legal_targets(target.filter, controller, &players, &cards, &zone_entities);
                if legal.is_empty() {
                    rejected.write(actions::Rejected::NoLegalTargets { spell });
                    if let Some(hand) =
                        helpers::find_zone(zones::Zone::Hand, Some(controller), &zone_entities)
                    {
                        commands.entity(spell).insert(zones::InZone(hand));
                    }
                    commands.entity(spell).remove::<stack::CastingStage>();
                    continue;
                }
                *pending = actions::PendingChoice(Some(actions::TargetChoice {
                    spell,
                    count: target.count,
                    legal,
                }));
                commands
                    .entity(spell)
                    .insert(stack::CastingStage::AwaitingTargets);
            }
            actions::Action::ChooseTargets(targets) => {
                let Some(choice) = (*pending).as_ref().cloned() else {
                    continue;
                };
                let chosen: BTreeSet<Entity> = targets.iter().copied().collect();
                let count_matches =
                    u32::try_from(targets.len()).unwrap_or(u32::MAX) == choice.count;
                let are_unique = chosen.len() == targets.len();
                let do_all_targets_are_legal = chosen.is_subset(&choice.legal);
                if !(count_matches && are_unique && do_all_targets_are_legal) {
                    rejected.write(actions::Rejected::InvalidTargets {
                        spell: choice.spell,
                    });
                    continue;
                }
                let Some(first) = targets.first() else {
                    rejected.write(actions::Rejected::InvalidTargets {
                        spell: choice.spell,
                    });
                    continue;
                };
                commands.entity(choice.spell).insert((
                    stack::ChosenTarget(*first),
                    stack::CastingStage::AwaitingPayment,
                ));
                *pending = actions::PendingChoice(None);
            }
            actions::Action::PassPriority => {}
        }
    }
}

fn legal_targets(
    filter: effects::Filter,
    controller: Entity,
    players: &Query<Entity, With<zones::Player>>,
    cards: &Query<(
        Entity,
        &cards::CardDefinition,
        &zones::Owner,
        &zones::InZone,
    )>,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) -> BTreeSet<Entity> {
    let all_players: Vec<Entity> = players.iter().collect();
    let allowed_controllers: Option<BTreeSet<Entity>> = filter.controlled_by.map(|player| {
        player
            .resolve(controller, &all_players)
            .into_iter()
            .collect()
    });
    let mut legal = BTreeSet::new();
    if filter.kind == effects::FilterKind::AnyTarget {
        legal.extend(all_players.iter().copied());
    }
    for (entity, definition, owner, in_zone) in cards.iter() {
        if !definition.types.contains(&cards::CardType::Creature) {
            continue;
        }
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Battlefield, zone_entities) {
            continue;
        }
        if let Some(allowed) = &allowed_controllers
            && !allowed.contains(&**owner)
        {
            continue;
        }
        if matches!(
            filter.kind,
            effects::FilterKind::Creature | effects::FilterKind::AnyTarget
        ) {
            legal.insert(entity);
        }
    }
    legal
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        assert!(game.target_choice().is_some());
    }

    #[test]
    fn test_bolt_lists_every_legal_target() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        assert_eq!(
            game.target_choice(),
            Some(testing::pick(1).from([bears, game.p1, game.p2]))
        );
    }

    #[test]
    fn test_spell_with_no_legal_target_cannot_be_cast() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::murder()])
            .mana_pool(testing::Seat::P1, mana![1, B, B])
            .build();
        let murder = game.find(game.p1, zones::Zone::Hand, "Murder").unwrap();
        game.act(game.p1, actions::Action::Cast(murder));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: murder })
        );
        assert_eq!(game.zone_of(murder), zones::Zone::Hand);
    }

    #[test]
    fn test_any_target_in_multiplayer() {
        let mut game = testing::MockGame::builder()
            .players(4)
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        assert_eq!(
            game.target_choice(),
            Some(testing::pick(1).from([game.p(1), game.p(2), game.p(3), game.p(4)]))
        );
    }
}
