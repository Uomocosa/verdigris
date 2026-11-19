use crate::{BATTLEFIELD, HAND, STACK};
use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, ResMut, With};

use crate::actions;
use crate::cards;
use crate::effects;
use crate::players;
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
    players: Query<Entity, With<players::Player>>,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for action in actions.read() {
        match &action.action {
            actions::Action::Cast(spell) => {
                let spell = *spell;
                let Ok((_, definition, owner, in_zone)) = cards.get(spell) else {
                    continue;
                };
                if !helpers::is_card_in_zone(**in_zone, STACK, &zone_entities) {
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
                let legal = legal_targets(
                    spell,
                    target.filter,
                    controller,
                    &players,
                    &cards,
                    &zone_entities,
                );
                if legal.is_empty() {
                    rejected.write(actions::Rejected::NoLegalTargets { spell });
                    if let Some(hand) = helpers::find_zone(HAND, Some(controller), &zone_entities) {
                        commands.entity(spell).insert(zones::InZone(hand));
                    }
                    commands.entity(spell).remove::<stack::CastingStage>();
                    continue;
                }
                *pending =
                    actions::PendingChoice(Some(actions::Choice::Target(actions::TargetChoice {
                        spell,
                        count: target.count,
                        legal,
                    })));
                commands
                    .entity(spell)
                    .insert(stack::CastingStage::AwaitingTargets);
            }
            actions::Action::ChooseTargets(targets) => {
                let Some(actions::Choice::Target(choice)) = (*pending).as_ref().cloned() else {
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
            actions::Action::ChooseDiscard(_) | actions::Action::PassPriority => {}
        }
    }
}

fn legal_targets(
    casting_spell: Entity,
    filter: effects::Filter,
    controller: Entity,
    players: &Query<Entity, With<players::Player>>,
    cards: &Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
    )>,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) -> BTreeSet<Entity> {
    let all_players: Vec<Entity> = players.iter().collect();
    let allowed_controllers: Option<BTreeSet<Entity>> = filter.controlled_by.map(|player| {
        player
            .resolve(controller, &all_players)
            .into_iter()
            .collect()
    });
    let mut legal = BTreeSet::new();
    if matches!(
        filter.kind,
        effects::FilterKind::AnyTarget | effects::FilterKind::Player
    ) {
        legal.extend(all_players.iter().copied());
    }
    for (entity, definition, owner, in_zone) in cards.iter() {
        if entity == casting_spell {
            continue;
        }
        if let Some(allowed) = &allowed_controllers
            && !allowed.contains(&**owner)
        {
            continue;
        }
        let is_legal = match filter.kind {
            effects::FilterKind::Spell
            | effects::FilterKind::CreatureSpell
            | effects::FilterKind::NoncreatureSpell
            | effects::FilterKind::InstantSpell => {
                helpers::is_card_in_zone(**in_zone, STACK, zone_entities)
                    && helpers::does_spell_match_filter(definition, filter)
            }
            effects::FilterKind::Creature | effects::FilterKind::AnyTarget => {
                helpers::does_creature_match_filter(definition, filter)
                    && helpers::is_card_in_zone(**in_zone, BATTLEFIELD, zone_entities)
            }
            effects::FilterKind::Player => false,
        };
        if is_legal {
            legal.insert(entity);
        }
    }
    legal
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND, STACK};

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([lightning_bolt()]).mana_pool(mana![R]),
            P2.battlefield([grizzly_bears()]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        game.act(p1, actions::Action::Cast(bolt));
        assert!(game.target_choice().is_some());
    }

    #[test]
    fn test_bolt_lists_every_legal_target() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([lightning_bolt()]).mana_pool(mana![R]),
            P2.battlefield([grizzly_bears()]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let bears = game
            .player(p2)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();
        game.act(p1, actions::Action::Cast(bolt));
        assert_eq!(game.target_choice(), Some(pick(1).from([bears, p1, p2])));
    }

    #[test]
    fn test_spell_with_no_legal_target_cannot_be_cast() {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([murder()]).mana_pool(mana![1, B, B]), P2]);
        let murder = game.player(p1).cards(HAND, MURDER).only().unwrap();
        game.act(p1, actions::Action::Cast(murder));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: murder })
        );
        assert_eq!(game.zone_of(murder), HAND);
    }

    #[test]
    fn doom_blade_lists_only_nonblack_creatures() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([doom_blade()]).mana_pool(mana![1, B]),
            P2.battlefield([grizzly_bears(), walking_corpse()]),
        ]);
        let blade = game.player(p1).cards(HAND, DOOM_BLADE).only().unwrap();
        let bears = game
            .player(p2)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();
        game.act(p1, actions::Action::Cast(blade));
        assert_eq!(game.target_choice(), Some(pick(1).from([bears])));
    }

    #[test]
    fn doom_blade_rejects_a_black_creature_as_target() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([doom_blade()]).mana_pool(mana![1, B]),
            P2.battlefield([grizzly_bears(), walking_corpse()]),
        ]);
        let blade = game.player(p1).cards(HAND, DOOM_BLADE).only().unwrap();
        let corpse = game
            .player(p2)
            .cards(BATTLEFIELD, WALKING_CORPSE)
            .only()
            .unwrap();
        game.act(p1, actions::Action::Cast(blade));
        game.act(p1, actions::Action::ChooseTargets(vec![corpse]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidTargets { spell: blade })
        );
    }

    #[test]
    fn doom_blade_cannot_be_cast_with_only_black_creatures() {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([doom_blade()]).mana_pool(mana![1, B]),
            P2.battlefield([walking_corpse()]),
        ]);
        let blade = game.player(p1).cards(HAND, DOOM_BLADE).only().unwrap();
        game.act(p1, actions::Action::Cast(blade));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: blade })
        );
        assert_eq!(game.zone_of(blade), HAND);
    }

    #[test]
    fn cancel_cannot_target_a_permanent() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([lightning_bolt(), cancel()])
                .mana_pool(mana![1, U, U, R]),
            P2.battlefield([grizzly_bears()]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let cancel = game.player(p1).cards(HAND, CANCEL).only().unwrap();
        let bears = game
            .player(p2)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![p2]));
        game.act(p1, actions::Action::Cast(cancel));

        assert_eq!(game.target_choice(), Some(pick(1).from([bolt])));

        game.act(p1, actions::Action::ChooseTargets(vec![bears]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidTargets { spell: cancel })
        );
    }

    #[test]
    fn essence_scatter_cannot_target_a_noncreature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([lightning_bolt()]).mana_pool(mana![R]),
            P2.hand([essence_scatter()]).mana_pool(mana![1, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let scatter = game.player(p2).cards(HAND, ESSENCE_SCATTER).only().unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(scatter));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: scatter })
        );
        assert_eq!(game.zone_of(scatter), HAND);
        assert_eq!(game.zone_of(bolt), STACK);
    }

    #[test]
    fn negate_cannot_target_a_creature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([grizzly_bears()]).mana_pool(mana![1, G]),
            P2.hand([negate()]).mana_pool(mana![1, U]),
        ]);
        let bears = game.player(p1).cards(HAND, GRIZZLY_BEARS).only().unwrap();
        let negate = game.player(p2).cards(HAND, NEGATE).only().unwrap();

        game.act(p1, actions::Action::Cast(bears));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(negate));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: negate })
        );
        assert_eq!(game.zone_of(negate), HAND);
        assert_eq!(game.zone_of(bears), STACK);
    }

    #[test]
    fn dispel_cannot_target_a_sorcery_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([divination()]).mana_pool(mana![2, U]),
            P2.hand([dispel()]).mana_pool(mana![U]),
        ]);
        let divination = game.player(p1).cards(HAND, DIVINATION).only().unwrap();
        let dispel = game.player(p2).cards(HAND, DISPEL).only().unwrap();

        game.act(p1, actions::Action::Cast(divination));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(dispel));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoLegalTargets { spell: dispel })
        );
        assert_eq!(game.zone_of(dispel), HAND);
        assert_eq!(game.zone_of(divination), STACK);
    }

    #[test]
    fn test_any_target_in_multiplayer() {
        let (mut game, [p1, p2, p3, p4]) =
            MockGame::new([P1.hand([lightning_bolt()]).mana_pool(mana![R]), P2, P3, P4]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        game.act(p1, actions::Action::Cast(bolt));
        assert_eq!(game.target_choice(), Some(pick(1).from([p1, p2, p3, p4])));
    }
}
