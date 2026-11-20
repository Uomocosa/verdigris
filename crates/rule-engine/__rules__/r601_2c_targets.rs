use crate::{BATTLEFIELD, HAND, STACK};
use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, ResMut, With};

use crate::actions;
use crate::cards;
use crate::effects;
use crate::players;
use crate::stack;
use crate::zones;

/// 601.2c The player announces their choice of an appropriate object or player for each target the
/// spell requires. A spell may require some targets only if an alternative or additional cost (such
/// as a kicker cost) or a particular mode was chosen for it; otherwise, the spell is cast as though
/// it did not require those targets. Similarly, a spell may require alternative targets only if an
/// alternative or additional cost was chosen for it. If the spell has a variable number of targets,
/// the player announces how many targets they will choose before they announce those targets. In
/// some cases, the number of targets will be defined by the spell’s text. Once the number of
/// targets the spell has is determined, that number doesn’t change, even if the information used to
/// determine the number of targets does. The same target can’t be chosen multiple times for any one
/// instance of the word “target” on the spell. However, if the spell uses the word “target” in
/// multiple places, the same object or player can be chosen once for each instance of the word
/// “target” (as long as it fits the targeting criteria). If any effects say that an object or
/// player must be chosen as a target, the player chooses targets so that they obey the maximum
/// possible number of such effects without violating any rules or effects that say that an object
/// or player can’t be chosen as a target. The chosen objects and/or players each become a target of
/// that spell. (Any abilities that trigger when those objects and/or players become the target of a
/// spell trigger at this point; they’ll wait to be put on the stack until the spell has finished
/// being cast.)
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
                if !STACK.contains_card(**in_zone, &zone_entities) {
                    continue;
                }
                let controller = **owner;
                let Some(effect) = definition.effect else {
                    commands.entity(spell).insert(stack::CastingStage::AwaitingPayment);
                    continue;
                };
                let Some(target) = effect.target else {
                    commands.entity(spell).insert(stack::CastingStage::AwaitingPayment);
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
                    if let Some(hand) = HAND.find_entity(Some(controller), &zone_entities) {
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
                commands.entity(spell).insert(stack::CastingStage::AwaitingTargets);
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
            actions::Action::ChooseDiscard(_)
            | actions::Action::AssignCombatDamage(_)
            | actions::Action::DeclareAttackers(_)
            | actions::Action::DeclareBlockers(_)
            | actions::Action::PlayLand(_)
            | actions::Action::ActivateManaAbility { .. }
            | actions::Action::PassPriority => {}
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
    let allowed_controllers: Option<BTreeSet<Entity>> = filter
        .controlled_by
        .map(|player| player.resolve(controller, &all_players).into_iter().collect());
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
                STACK.contains_card(**in_zone, zone_entities) && filter.does_spell_match(definition)
            }
            effects::FilterKind::Creature | effects::FilterKind::AnyTarget => {
                filter.does_creature_match(definition)
                    && BATTLEFIELD.contains_card(**in_zone, zone_entities)
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
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt)])?;
        assert!(game.target_choice().is_some());
        Ok(())
    }

    #[test]
    fn r601_2c_bolt_lists_every_legal_target() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bolt)])?;
        assert_eq!(
            game.target_choice(),
            Some(pick(1).from([bears.into(), *p1, *p2]))
        );
        Ok(())
    }

    #[test]
    fn r601_2c_spell_with_no_legal_target_cannot_be_cast() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([MURDER]).mana_pool(mana![1, B, B]), P2]);
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let err = game.play([p1.cast(murder)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: murder.into()
            })
        );
        assert_eq!(game.zone_of(murder), HAND);
        Ok(())
    }

    #[test]
    fn r601_2c_doom_blade_lists_only_nonblack_creatures() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([GRIZZLY_BEARS, WALKING_CORPSE]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(blade)])?;
        assert_eq!(game.target_choice(), Some(pick(1).from([bears.into()])));
        Ok(())
    }

    #[test]
    fn r601_2c_doom_blade_rejects_a_black_creature_as_target() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([GRIZZLY_BEARS, WALKING_CORPSE]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [corpse] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        let err = game.play([p1.cast(blade).targeting(corpse)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::InvalidTargets {
                spell: blade.into()
            })
        );
        Ok(())
    }

    #[test]
    fn r601_2c_doom_blade_cannot_be_cast_with_only_black_creatures() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([WALKING_CORPSE]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let err = game.play([p1.cast(blade)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: blade.into()
            })
        );
        assert_eq!(game.zone_of(blade), HAND);
        Ok(())
    }

    #[test]
    fn r601_2c_cancel_cannot_target_a_permanent() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, CANCEL]).mana_pool(mana![1, U, U, R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [cancel] = game.player(p1).cards_in(HAND).exactly::<1>(CANCEL)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.cast(bolt).targeting(p2), p1.cast(cancel)])?;

        assert_eq!(game.target_choice(), Some(pick(1).from([bolt.into()])));

        game.act(p1, actions::Action::ChooseTargets(vec![bears.into()]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidTargets {
                spell: cancel.into()
            })
        );
        Ok(())
    }

    #[test]
    fn r601_2c_essence_scatter_cannot_target_a_noncreature_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([ESSENCE_SCATTER]).mana_pool(mana![1, U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [scatter] = game.player(p2).cards_in(HAND).exactly::<1>(ESSENCE_SCATTER)?;

        let err = game
            .play([
                p1.cast(bolt).targeting(p2).pass_priority(),
                p2.cast(scatter),
            ])
            .unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: scatter.into()
            })
        );
        assert_eq!(game.zone_of(scatter), HAND);
        assert_eq!(game.zone_of(bolt), STACK);
        Ok(())
    }

    #[test]
    fn r601_2c_negate_cannot_target_a_creature_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [negate] = game.player(p2).cards_in(HAND).exactly::<1>(NEGATE)?;

        let err = game.play([p1.cast(bears).pass_priority(), p2.cast(negate)]).unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: negate.into()
            })
        );
        assert_eq!(game.zone_of(negate), HAND);
        assert_eq!(game.zone_of(bears), STACK);
        Ok(())
    }

    #[test]
    fn r601_2c_dispel_cannot_target_a_sorcery_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DIVINATION]).mana_pool(mana![2, U]),
            P2.hand([DISPEL]).mana_pool(mana![U]),
        ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;
        let [dispel] = game.player(p2).cards_in(HAND).exactly::<1>(DISPEL)?;

        let err = game.play([p1.cast(divination).pass_priority(), p2.cast(dispel)]).unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: dispel.into()
            })
        );
        assert_eq!(game.zone_of(dispel), HAND);
        assert_eq!(game.zone_of(divination), STACK);
        Ok(())
    }

    #[test]
    fn r601_2c_any_target_in_multiplayer() -> TestResult {
        let (mut game, [p1, p2, p3, p4]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2, P3, P4]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt)])?;
        assert_eq!(
            game.target_choice(),
            Some(pick(1).from([*p1, *p2, *p3, *p4]))
        );
        Ok(())
    }
}
