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
        let player_targets =
            chosen_target.filter(|target| lookups.lives.get(*target).is_ok()).map_or_else(
                || resolve_player_targets(&lookups, effect, controller),
                |player| vec![player],
            );
        let target_zone = chosen_target.and_then(|target| lookups.in_zone.get(target).ok());
        let target_on_battlefield =
            target_zone.is_some_and(|zone| zones.contains(BATTLEFIELD, **zone));
        let target_on_stack = target_zone.is_some_and(|zone| zones.contains(STACK, **zone));
        for kind in std::iter::once(effect.kind).chain(effect.follow_ups.into_iter().flatten()) {
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
                    for player in players_or_fallback(&player_targets, **owner) {
                        draws.write(players::DrawRequested { player, count });
                    }
                }
                effects::EffectKind::Mill(count) => {
                    for player in players_or_fallback(&player_targets, controller) {
                        effect_messages.p2().write(effects::MillRequested { player, count });
                    }
                }
                effects::EffectKind::LoseLife(amount) => {
                    request_life_loss(&mut player_effects.p0(), &player_targets, amount);
                }
                effects::EffectKind::Poison(amount) => {
                    for player in &player_targets {
                        add_poison_counters(&mut commands, &lookups, *player, amount);
                    }
                }
                effects::EffectKind::GainLife(amount) => {
                    for player in players_or_fallback(&player_targets, controller) {
                        player_effects.p2().write(players::LifeGained { player, amount });
                    }
                }
                effects::EffectKind::Discard(count) => {
                    request_discard(&mut player_effects.p1(), &player_targets, count);
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

fn request_life_loss(
    life_lost: &mut MessageWriter<players::LifeLost>,
    players: &[Entity],
    amount: u32,
) {
    for player in players {
        life_lost.write(players::LifeLost {
            player: *player,
            amount,
        });
    }
}

fn request_discard(
    discarded: &mut MessageWriter<players::DiscardRequested>,
    players: &[Entity],
    count: u32,
) {
    for player in players {
        discarded.write(players::DiscardRequested {
            player: *player,
            count,
        });
    }
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
