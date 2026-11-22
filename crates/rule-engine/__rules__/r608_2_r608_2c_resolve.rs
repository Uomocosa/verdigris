use crate::{BATTLEFIELD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, ParamSet};

use crate::damage;
use crate::effects;
use crate::players;
use crate::stack;
use crate::zones;

type EffectMessageWriters<'a, 'b, 'c, 'd> = (
    MessageWriter<'a, effects::PowerToughnessModified>,
    MessageWriter<'b, effects::KeywordGranted>,
    MessageWriter<'c, effects::MillRequested>,
    MessageWriter<'d, effects::TokensRequested>,
);

/// 608.2. If the object that’s resolving is an instant spell, a sorcery spell, or an ability, its
/// resolution may involve several steps. The steps described in rules 608.2a and 608.2b are
/// followed first. The steps described in rules 608.2c–m are then followed as appropriate, in no
/// specific order. The steps described in rule 608.2n and 608.2p are followed last.
///
/// 608.2c The controller of the spell or ability follows its instructions in the order written.
/// However, replacement effects may modify these actions. In some cases, later text on the card may
/// modify the meaning of earlier text (for example, “Destroy target creature. It can’t be
/// regenerated” or “Counter target spell. If that spell is countered this way, put it on top of its
/// owner’s library instead of into its owner’s graveyard.”) Don’t just apply effects step by step
/// without thinking in these cases—read the whole text and apply the rules of English to the text.
pub fn r608_2_r608_2c_resolve(
    mut commands: Commands,
    mut resolutions: MessageReader<stack::Resolution>,
    mut damage_dealt: MessageWriter<damage::DamageDealt>,
    mut destructions: MessageWriter<damage::Destruction>,
    mut countered: MessageWriter<stack::Countered>,
    mut draws: MessageWriter<players::DrawRequested>,
    mut player_effects: ParamSet<(
        MessageWriter<players::LifeLost>,
        MessageWriter<players::DiscardRequested>,
        MessageWriter<players::LifeGained>,
    )>,
    mut effect_messages: ParamSet<EffectMessageWriters<'_, '_, '_, '_>>,
    lookups: stack::ResolutionLookups<'_, '_>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        if !lookups.in_zone.get(spell).is_ok_and(|zone| zones.contains(STACK, **zone)) {
            continue;
        }
        let Some(effect) = effect_of(&lookups, spell) else {
            continue;
        };
        let Ok(owner) = lookups.owners.get(spell) else {
            continue;
        };
        let controller = lookups.abilities.get(spell).map_or(**owner, |ability| ability.controller);
        let chosen_target = lookups.chosen_targets.get(spell).ok().map(|chosen| **chosen);
        let player_target = chosen_target.filter(|target| lookups.lives.get(*target).is_ok());
        let target_zone = chosen_target.and_then(|target| lookups.in_zone.get(target).ok());
        let target_on_battlefield =
            target_zone.is_some_and(|zone| zones.contains(BATTLEFIELD, **zone));
        let target_on_stack = target_zone.is_some_and(|zone| zones.contains(STACK, **zone));
        for kind in [Some(effect.kind), effect.then, effect.finally].into_iter().flatten() {
            match kind {
                effects::EffectKind::Damage(amount) => {
                    request_damage(&mut damage_dealt, &lookups, spell, chosen_target, amount);
                }
                effects::EffectKind::Destroy => {
                    if let Some(target) = chosen_target
                        && target_on_battlefield
                    {
                        destructions.write(damage::Destruction(target));
                    }
                }
                effects::EffectKind::Counter => {
                    if let Some(target) = chosen_target
                        && target_on_stack
                    {
                        countered.write(stack::Countered(target));
                    }
                }
                effects::EffectKind::CreateTokens { count, definition } => {
                    effect_messages.p3().write(effects::TokensRequested {
                        count,
                        token: definition,
                        controller,
                    });
                }
                effects::EffectKind::Draw(count) => {
                    draws.write(players::DrawRequested {
                        player: player_target.unwrap_or(**owner),
                        count,
                    });
                }
                effects::EffectKind::Mill(count) => {
                    request_mill(&mut effect_messages.p2(), player_target, controller, count);
                }
                effects::EffectKind::LoseLife(amount) => {
                    if let Some(player) = player_target {
                        player_effects.p0().write(players::LifeLost { player, amount });
                    }
                }
                effects::EffectKind::Poison(amount) => {
                    add_poison_counters(&mut commands, &lookups, player_target, amount);
                }
                effects::EffectKind::GainLife(amount) => {
                    player_effects.p2().write(players::LifeGained {
                        player: player_target.unwrap_or(controller),
                        amount,
                    });
                }
                effects::EffectKind::Discard(count) => {
                    if let Some(player) = player_target {
                        player_effects.p1().write(players::DiscardRequested { player, count });
                    }
                }
                effects::EffectKind::ModifyPowerToughness { power, toughness } => {
                    if let Some(target) = chosen_target
                        && target_on_battlefield
                    {
                        effect_messages.p0().write(effects::PowerToughnessModified {
                            target,
                            power,
                            toughness,
                        });
                    }
                }
                effects::EffectKind::GainKeyword(keyword) => {
                    if let Some(target) = chosen_target
                        && target_on_battlefield
                    {
                        effect_messages.p1().write(effects::KeywordGranted { target, keyword });
                    }
                }
                effects::EffectKind::WinTheGame => {
                    commands.entity(controller).insert(players::Won);
                }
            }
        }
    }
}

fn effect_of(lookups: &stack::ResolutionLookups<'_, '_>, spell: Entity) -> Option<effects::Effect> {
    lookups
        .definitions
        .get(spell)
        .ok()
        .and_then(|definition| definition.effect)
        .or_else(|| lookups.abilities.get(spell).ok().map(|ability| ability.effect))
}

fn request_damage(
    damage_dealt: &mut MessageWriter<damage::DamageDealt>,
    lookups: &stack::ResolutionLookups<'_, '_>,
    spell: Entity,
    target: Option<Entity>,
    amount: u32,
) {
    let Some(target) = target else {
        return;
    };
    if lookups.lives.get(target).is_err()
        && lookups.definitions.get(target).is_err()
        && lookups.abilities.get(target).is_err()
    {
        return;
    }
    damage_dealt.write(damage::DamageDealt {
        source: spell,
        target,
        amount,
    });
}

fn request_mill(
    mills: &mut MessageWriter<effects::MillRequested>,
    player_target: Option<Entity>,
    controller: Entity,
    count: u32,
) {
    mills.write(effects::MillRequested {
        player: player_target.unwrap_or(controller),
        count,
    });
}

fn add_poison_counters(
    commands: &mut Commands,
    lookups: &stack::ResolutionLookups<'_, '_>,
    player: Option<Entity>,
    amount: u32,
) {
    let Some(player) = player else {
        return;
    };
    let counters =
        lookups.poison.get(player).map_or(0, |counters| **counters).saturating_add(amount);
    commands.entity(player).insert(players::PoisonCounters(counters));
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn bolt_deals_damage_to_a_player() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2).end_turn(), p2.end_turn()])?;
        assert_eq!(game.life(p2), 17);
        Ok(())
    }

    #[test]
    fn bolt_kills_bears() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(bears).end_turn(), p2.end_turn()])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        let [bolt] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        Ok(())
    }

    #[test]
    fn murder_kills_bears() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MURDER]).mana_pool(mana![1, B, B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        game.play([p1.cast(murder).targeting(bears).end_turn(), p2.end_turn()])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        Ok(())
    }

    #[test]
    fn doom_blade_destroys_a_green_creature() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [doom_blade] = game.player(p1).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        game.play([
            p1.cast(doom_blade).targeting(bears).end_turn(),
            p2.end_turn(),
        ])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        let [blade] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(DOOM_BLADE)?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(blade));
        Ok(())
    }
}
