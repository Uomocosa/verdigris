use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query};

use crate::actions;
use crate::cards;
use crate::effects;
use crate::stack;
use crate::zones;
use crate::{HAND, STACK};

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
///
/// # See also
///
/// 702.11b “Hexproof” on a permanent means “This permanent can’t be the target of spells or
/// abilities your opponents control.”
pub fn r601_2c_targets() -> ScheduleConfigs<ScheduleSystem> {
    (ask_for_targets, take_the_chosen_targets).chain()
}

fn ask_for_targets(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut asks: MessageWriter<actions::Ask>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
    )>,
    zones: zones::ZoneLookup,
    targeting: stack::Targeting,
) {
    for action in actions.read() {
        let actions::Action::Cast(spell) = &action.action else {
            continue;
        };
        let spell = *spell;
        let Ok((_, definition, owner, in_zone)) = cards.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        let controller = **owner;
        let target = definition
            .spell_ability()
            .and_then(|effect| effect.target)
            .or_else(|| definition.enchant().map(|filter| effects::target(1, filter)));
        let Some(target) = target else {
            commands.entity(spell).insert(stack::CastingStage::AwaitingPayment);
            continue;
        };
        let legal = targeting.legal_targets(spell, target, controller);
        if legal.is_empty() {
            rejected.write(actions::Rejected::NoLegalTargets { spell });
            if let Some(hand) = zones.find(HAND, Some(controller)) {
                commands.entity(spell).insert(zones::InZone(hand));
            }
            commands.entity(spell).remove::<stack::CastingStage>();
            continue;
        }
        asks.write(actions::Ask(actions::Choice::Target(
            actions::TargetChoice {
                player: controller,
                spell,
                count: target.count,
                legal,
            },
        )));
        commands.entity(spell).insert(stack::CastingStage::AwaitingTargets);
    }
}

fn take_the_chosen_targets(mut answers: MessageReader<actions::Answered>, mut commands: Commands) {
    for answered in answers.read() {
        let actions::Choice::Target(choice) = &answered.choice else {
            continue;
        };
        let actions::Action::ChooseTargets(targets) = &answered.answer else {
            continue;
        };
        let Some(first) = targets.first() else {
            continue;
        };
        commands.entity(choice.spell).insert((
            stack::ChosenTarget(*first),
            stack::CastingStage::AwaitingPayment,
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn bolt_lists_every_legal_target() -> TestResult {
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
    fn spell_with_no_legal_target_cannot_be_cast() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([MURDER]).mana_pool(mana![1, B, B]), P2]);
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        let err = game.play([p1.cast(murder)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: murder.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(murder));
        Ok(())
    }

    #[test]
    fn doom_blade_lists_only_nonblack_creatures() -> TestResult {
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
    fn doom_blade_rejects_a_black_creature_as_target() -> TestResult {
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
    fn doom_blade_cannot_be_cast_with_only_black_creatures() -> TestResult {
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
        assert!(game.player(p1).cards_in(HAND).has(blade));
        Ok(())
    }

    #[test]
    fn cancel_cannot_target_a_permanent() -> TestResult {
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
    fn essence_scatter_cannot_target_a_noncreature_spell() -> TestResult {
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
        assert!(game.player(p2).cards_in(HAND).has(scatter));
        assert!(game.stack().has(bolt));
        Ok(())
    }

    #[test]
    fn negate_cannot_target_a_creature_spell() -> TestResult {
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
        assert!(game.player(p2).cards_in(HAND).has(negate));
        assert!(game.stack().has(bears));
        Ok(())
    }

    #[test]
    fn dispel_cannot_target_a_sorcery_spell() -> TestResult {
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
        assert!(game.player(p2).cards_in(HAND).has(dispel));
        assert!(game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn any_target_in_multiplayer() -> TestResult {
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

    #[test]
    fn an_opponent_cannot_target_a_creature_with_hexproof() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.hand([RANGERS_GUILE])
                .mana_pool(mana![G])
                .battlefield([GLADECOVER_SCOUT, GRIZZLY_BEARS]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [guile] = game.player(p2).cards_in(HAND).exactly::<1>(RANGERS_GUILE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.pass_priority(),
            p2.cast(guile).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(bears));

        let err = game.play([p1.cast(blade)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoLegalTargets {
                spell: blade.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(blade));
        Ok(())
    }

    #[test]
    fn a_controller_can_target_their_own_creature_with_hexproof() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]).battlefield([GLADECOVER_SCOUT]),
            P2,
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [scout] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GLADECOVER_SCOUT)?;
        game.play([
            p1.cast(blade).targeting(scout).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p1.arrange_graveyard([scout, blade])])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(scout));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(blade));
        Ok(())
    }
}
