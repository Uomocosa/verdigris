use crate::{BATTLEFIELD, EXILE, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, ParamSet};

use crate::effects;
use crate::keywords;
use crate::players;
use crate::rules;
use crate::stack;
use crate::zones;

type EffectMessageWriters<'a, 'b, 'c, 'd> = (
    MessageWriter<'a, effects::PowerToughnessModified>,
    MessageWriter<'b, effects::KeywordGranted>,
    MessageWriter<'c, rules::Would<rules::Mill>>,
    MessageWriter<'d, rules::Would<rules::CreateTokens>>,
);

type PlayerEffectWriters<'a, 'b, 'c, 'd, 'e> = (
    MessageWriter<'a, rules::Would<rules::LoseLife>>,
    MessageWriter<'b, rules::Would<rules::Discard>>,
    MessageWriter<'c, rules::Would<rules::GainLife>>,
    MessageWriter<'d, rules::Would<rules::Draw>>,
    MessageWriter<'e, rules::Would<rules::WinGame>>,
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
    mut damage_dealt: MessageWriter<rules::Would<rules::DealDamage>>,
    mut destructions: MessageWriter<rules::Would<rules::Destroy>>,
    mut countered: MessageWriter<rules::Would<rules::Counter>>,
    mut player_effects: ParamSet<PlayerEffectWriters<'_, '_, '_, '_, '_>>,
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
        let player_targets =
            chosen_target.filter(|target| lookups.lives.get(*target).is_ok()).map_or_else(
                || resolve_player_targets(&lookups, effect, *controller),
                |p| vec![p],
            );
        let target_zone = chosen_target.and_then(|target| lookups.in_zone.get(target).ok());
        let target_on_stack = target_zone.is_some_and(|zone| zones.contains(STACK, **zone));
        let battlefield_target = chosen_target
            .filter(|_| target_zone.is_some_and(|zone| zones.contains(BATTLEFIELD, **zone)))
            .or_else(|| self_target_on_battlefield(&lookups, spell, effect, &zones));
        for kind in std::iter::once(effect.kind).chain(effect.follow_ups.into_iter().flatten()) {
            match kind {
                effects::EffectKind::Damage(amount) => {
                    request_damage(
                        &mut damage_dealt,
                        &lookups,
                        spell,
                        chosen_target,
                        *controller,
                        amount,
                    );
                }
                effects::EffectKind::Destroy => {
                    request_destroy(&mut destructions, &lookups, battlefield_target, *controller);
                }
                #[rustfmt::skip]
                effects::EffectKind::Exile => {
                    request_exile(&mut commands, &lookups, &zones, spell, chosen_target);
                }
                effects::EffectKind::Counter => {
                    request_counter(
                        &mut countered,
                        &lookups,
                        chosen_target,
                        target_on_stack,
                        *controller,
                    );
                }
                effects::EffectKind::CreateTokens { count, definition } => {
                    request_tokens(&mut effect_messages.p3(), *controller, count, definition);
                }
                effects::EffectKind::Draw(count) => {
                    request_draws(&mut player_effects.p3(), &player_targets, ***owner, count);
                }
                effects::EffectKind::Mill(count) => {
                    request_mills(
                        &mut effect_messages.p2(),
                        &player_targets,
                        *controller,
                        count,
                    );
                }
                effects::EffectKind::LoseLife(amount) => {
                    request_life_loss(&mut player_effects.p0(), &player_targets, amount);
                }
                effects::EffectKind::Poison(amount) => {
                    request_poison(&mut commands, &lookups, &player_targets, amount);
                }
                effects::EffectKind::GainLife(amount) => {
                    request_life_gain(
                        &mut player_effects.p2(),
                        &player_targets,
                        *controller,
                        amount,
                    );
                }
                effects::EffectKind::Discard(count) => {
                    request_discard(&mut player_effects.p1(), &player_targets, count);
                }
                effects::EffectKind::ModifyPowerToughness { power, toughness } => {
                    request_modify_power_toughness(
                        &mut effect_messages,
                        battlefield_target,
                        power,
                        toughness,
                    );
                }
                effects::EffectKind::GainKeyword(keyword) => {
                    request_gain_keyword(&mut effect_messages, battlefield_target, keyword);
                }
                effects::EffectKind::WinTheGame => {
                    let winner = controller;
                    player_effects.p4().write(rules::Would(rules::WinGame(winner)));
                }
            }
        }
    }
}

fn self_target_on_battlefield(
    lookups: &stack::ResolutionLookups<'_, '_>,
    spell: Entity,
    effect: effects::Effect,
    zones: &zones::ZoneLookup,
) -> Option<Entity> {
    if !effect
        .target
        .is_some_and(|target| target.filter.object == Some(effects::ObjectRelation::ThisObject))
    {
        return None;
    }
    let source = lookups.abilities.get(spell).ok().map(|ability| ability.source)?;
    let zone = lookups.in_zone.get(source).ok().copied()?;
    if zones.contains(BATTLEFIELD, *zone) {
        Some(source)
    } else {
        None
    }
}

fn effect_of(lookups: &stack::ResolutionLookups<'_, '_>, spell: Entity) -> Option<effects::Effect> {
    lookups
        .definitions
        .get(spell)
        .ok()
        .and_then(|definition| definition.spell_ability().copied())
        .or_else(|| lookups.abilities.get(spell).ok().map(|ability| ability.effect))
}

fn request_destroy(
    destructions: &mut MessageWriter<rules::Would<rules::Destroy>>,
    lookups: &stack::ResolutionLookups<'_, '_>,
    target: Option<Entity>,
    controller: Entity,
) {
    if let Some(target) = target {
        destructions.write(rules::Would(rules::Destroy {
            permanent: target,
            controller: players::PlayerId(permanent_controller(lookups, target, controller)),
        }));
    }
}

fn request_counter(
    countered: &mut MessageWriter<rules::Would<rules::Counter>>,
    lookups: &stack::ResolutionLookups<'_, '_>,
    target: Option<Entity>,
    target_on_stack: bool,
    controller: Entity,
) {
    if let Some(target) = target
        && target_on_stack
    {
        countered.write(rules::Would(rules::Counter {
            spell: target,
            controller: players::PlayerId(permanent_controller(lookups, target, controller)),
        }));
    }
}

fn request_exile(
    commands: &mut Commands,
    lookups: &stack::ResolutionLookups<'_, '_>,
    zones: &zones::ZoneLookup,
    spell: Entity,
    chosen_target: Option<Entity>,
) {
    let source =
        chosen_target.or_else(|| lookups.abilities.get(spell).ok().map(|ability| ability.source));
    let exile = source
        .and_then(|target| lookups.owners.get(target).ok())
        .and_then(|owner| zones.find(EXILE, Some(**owner)))
        .zip(source);
    if let Some((exile, target)) = exile {
        commands.entity(target).insert(zones::InZone(exile));
    }
}

fn request_modify_power_toughness(
    effect_messages: &mut ParamSet<EffectMessageWriters<'_, '_, '_, '_>>,
    target: Option<Entity>,
    power: i32,
    toughness: i32,
) {
    if let Some(target) = target {
        effect_messages.p0().write(effects::PowerToughnessModified {
            target,
            power,
            toughness,
        });
    }
}

fn request_gain_keyword(
    effect_messages: &mut ParamSet<EffectMessageWriters<'_, '_, '_, '_>>,
    target: Option<Entity>,
    keyword: keywords::StaticKeyword,
) {
    if let Some(target) = target {
        effect_messages.p1().write(effects::KeywordGranted { target, keyword });
    }
}

fn request_poison(
    commands: &mut Commands,
    lookups: &stack::ResolutionLookups<'_, '_>,
    players: &[Entity],
    amount: u32,
) {
    for player in players {
        add_poison_counters(commands, lookups, *player, amount);
    }
}

fn request_damage(
    damage_dealt: &mut MessageWriter<rules::Would<rules::DealDamage>>,
    lookups: &stack::ResolutionLookups<'_, '_>,
    spell: Entity,
    target: Option<Entity>,
    controller: Entity,
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
    damage_dealt.write(rules::Would(rules::DealDamage {
        source: spell,
        recipient: target,
        amount,
        controller: players::PlayerId(permanent_controller(lookups, target, controller)),
    }));
}

fn permanent_controller(
    lookups: &stack::ResolutionLookups<'_, '_>,
    target: Entity,
    fallback: Entity,
) -> Entity {
    if lookups.lives.get(target).is_ok() {
        return target;
    }
    lookups.owners.get(target).map_or(fallback, |owner| ***owner)
}

fn request_draws(
    draws: &mut MessageWriter<rules::Would<rules::Draw>>,
    players: &[Entity],
    fallback: Entity,
    count: u32,
) {
    for player in players_or_fallback(players, fallback) {
        draws.write(rules::Would(rules::Draw {
            player: players::PlayerId(player),
            amount: count,
        }));
    }
}

fn request_life_loss(
    life_lost: &mut MessageWriter<rules::Would<rules::LoseLife>>,
    players: &[Entity],
    amount: u32,
) {
    for player in players {
        life_lost.write(rules::Would(rules::LoseLife {
            player: players::PlayerId(*player),
            amount,
        }));
    }
}

fn request_discard(
    discarded: &mut MessageWriter<rules::Would<rules::Discard>>,
    players: &[Entity],
    count: u32,
) {
    for player in players {
        discarded.write(rules::Would(rules::Discard {
            player: players::PlayerId(*player),
            amount: count,
        }));
    }
}

fn request_mills(
    mills: &mut MessageWriter<rules::Would<rules::Mill>>,
    players: &[Entity],
    fallback: Entity,
    count: u32,
) {
    for player in players_or_fallback(players, fallback) {
        mills.write(rules::Would(rules::Mill {
            player: players::PlayerId(player),
            amount: count,
        }));
    }
}

fn request_life_gain(
    gained: &mut MessageWriter<rules::Would<rules::GainLife>>,
    players: &[Entity],
    fallback: Entity,
    amount: u32,
) {
    for player in players_or_fallback(players, fallback) {
        gained.write(rules::Would(rules::GainLife {
            player: players::PlayerId(player),
            amount,
        }));
    }
}

fn request_tokens(
    tokens: &mut MessageWriter<rules::Would<rules::CreateTokens>>,
    controller: Entity,
    count: u32,
    definition: effects::TokenDefinition,
) {
    tokens.write(rules::Would(rules::CreateTokens {
        controller: players::PlayerId(controller),
        amount: count,
        token: definition,
    }));
}

fn players_or_fallback(players: &[Entity], fallback: Entity) -> Vec<Entity> {
    if players.is_empty() {
        vec![fallback]
    } else {
        players.to_vec()
    }
}

fn resolve_player_targets(
    lookups: &stack::ResolutionLookups<'_, '_>,
    effect: effects::Effect,
    controller: Entity,
) -> Vec<Entity> {
    let Some(player) = effect.target.and_then(|target| target.filter.controlled_by) else {
        return Vec::new();
    };
    let players: Vec<Entity> = lookups.lives.iter().map(|(player, _)| player).collect();
    player.resolve(controller, &players)
}

fn add_poison_counters(
    commands: &mut Commands,
    lookups: &stack::ResolutionLookups<'_, '_>,
    player: Entity,
    amount: u32,
) {
    let counters =
        lookups.poison.get(player).map_or(0, |counters| **counters).saturating_add(amount);
    commands.entity(player).insert(players::PoisonCounters(counters));
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn each_opponent_draws_at_the_beginning_of_master_of_the_feasts_upkeep() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::Untap)
            .players([P1.battlefield([MASTER_OF_THE_FEAST]), P2]);
        let p1_hand_before = game.player(p1).cards_in(HAND).count();
        let p2_hand_before = game.player(p2).cards_in(HAND).count();

        game.play([p1.pass_priority(), p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.player(p1).cards_in(HAND).count(), p1_hand_before);
        assert_eq!(game.player(p2).cards_in(HAND).count(), p2_hand_before + 1);
        Ok(())
    }

    #[test]
    fn in_a_three_player_game_both_opponents_draw_at_masters_upkeep() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::builder().turn(1, turns::Step::Untap).players([
            P1.battlefield([MASTER_OF_THE_FEAST]),
            P2,
            P3,
        ]);
        let p1_hand_before = game.player(p1).cards_in(HAND).count();
        let p2_hand_before = game.player(p2).cards_in(HAND).count();
        let p3_hand_before = game.player(p3).cards_in(HAND).count();

        game.play([
            p1.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
            p3.pass_priority(),
        ])?;

        assert_eq!(game.player(p1).cards_in(HAND).count(), p1_hand_before);
        assert_eq!(game.player(p2).cards_in(HAND).count(), p2_hand_before + 1);
        assert_eq!(game.player(p3).cards_in(HAND).count(), p3_hand_before + 1);
        Ok(())
    }

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
