use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, Has, MessageReader, Query};

use crate::cards;
use crate::stack;
use crate::zones;

type FizzlingSpell<'w, 's> = Query<
    'w,
    's,
    (
        &'static cards::CardDefinition,
        &'static cards::Controller,
        &'static cards::Owner,
        &'static zones::InZone,
        Option<&'static stack::ChosenTarget>,
        Has<cards::Token>,
    ),
>;

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
    cards: FizzlingSpell<'_, '_>,
    zones: zones::ZoneLookup,
    targeting: stack::Targeting,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, controller, owner, in_zone, chosen, has_token)) = cards.get(spell)
        else {
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
            targeting.is_legal_target(spell, requirement, **controller, target)
        });
        if is_legal {
            continue;
        }
        if let Some(graveyard) = zones.find(GRAVEYARD, Some(**owner)) {
            zones::move_object(
                &mut commands,
                spell,
                definition.clone(),
                *owner,
                has_token.then_some(cards::Token),
                graveyard,
            );
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
        assert!(game.moved(bolt).to(GRAVEYARD).of(p1));
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
        assert!(game.moved(bears).to(GRAVEYARD).of(p2));
        assert!(game.moved(murder).to(GRAVEYARD).of(p1));
        assert!(game.stack().has(LIGHTNING_BOLT));

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.moved(bolt).to(GRAVEYARD).of(p1));
        assert_eq!(game.life(p2), 20);
        Ok(())
    }

    #[test]
    fn spell_fizzles_when_its_target_gains_hexproof_in_response() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.hand([RANGERS_GUILE]).mana_pool(mana![G]).battlefield([GRIZZLY_BEARS]),
        ]);
        let [blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        let [guile] = game.player(p2).cards_in(HAND).exactly::<1>(RANGERS_GUILE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.cast(blade).targeting(bears).pass_priority(),
            p2.cast(guile).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert!(game.moved(blade).to(GRAVEYARD).of(p1));
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
        assert!(game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert!(game.moved(blade).to(GRAVEYARD).of(p1));
        Ok(())
    }
}
