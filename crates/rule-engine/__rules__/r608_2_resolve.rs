use crate::{BATTLEFIELD, STACK};
use bevy::prelude::{Entity, MessageReader, MessageWriter, ParamSet, Query};

use crate::cards;
use crate::damage;
use crate::effects;
use crate::players;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 608.2 If the object that's resolving is an instant spell, a sorcery spell, or an ability, its
/// resolution may involve several steps. ... The steps described in rule 608.2n are followed
/// last.
///
/// 608.2c The controller of the spell or ability follows its instructions in the order written.
pub fn r608_2_resolve(
    mut resolutions: MessageReader<stack::Resolution>,
    mut damage_dealt: MessageWriter<damage::DamageDealt>,
    mut destructions: MessageWriter<damage::Destruction>,
    mut countered: MessageWriter<stack::Countered>,
    mut draws: MessageWriter<players::DrawRequested>,
    mut player_effects: ParamSet<(
        MessageWriter<players::LifeLost>,
        MessageWriter<players::DiscardRequested>,
    )>,
    mut power_toughness_modified: MessageWriter<effects::PowerToughnessModified>,
    spells: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    lives: Query<&players::Life>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone, chosen)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, STACK, &zone_entities) {
            continue;
        }
        let Some(effect) = definition.effect else {
            continue;
        };
        let chosen_target = chosen.map(|chosen| **chosen);
        let player_target = chosen_target.filter(|target| lives.get(*target).is_ok());
        for kind in [Some(effect.kind), effect.then].into_iter().flatten() {
            match kind {
                effects::EffectKind::Damage(amount) => {
                    if let Some(target) = chosen_target
                        && (lives.get(target).is_ok() || spells.get(target).is_ok())
                    {
                        damage_dealt.write(damage::DamageDealt {
                            source: spell,
                            target,
                            amount,
                        });
                    }
                }
                effects::EffectKind::Destroy => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(**target_zone, BATTLEFIELD, &zone_entities)
                    {
                        destructions.write(damage::Destruction(target));
                    }
                }
                effects::EffectKind::Counter => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(**target_zone, STACK, &zone_entities)
                    {
                        countered.write(stack::Countered(target));
                    }
                }
                effects::EffectKind::Draw(count) => {
                    draws.write(players::DrawRequested {
                        player: player_target.unwrap_or(**owner),
                        count,
                    });
                }
                effects::EffectKind::LoseLife(amount) => {
                    if let Some(player) = player_target {
                        player_effects.p0().write(players::LifeLost { player, amount });
                    }
                }
                effects::EffectKind::Discard(count) => {
                    if let Some(player) = player_target {
                        player_effects.p1().write(players::DiscardRequested { player, count });
                    }
                }
                effects::EffectKind::ModifyPowerToughness { power, toughness } => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(**target_zone, BATTLEFIELD, &zone_entities)
                    {
                        power_toughness_modified.write(effects::PowerToughnessModified {
                            target,
                            power,
                            toughness,
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2).end_turn(), p2.end_turn()])?;
        assert_eq!(game.life(p2), 17);
        Ok(())
    }

    #[test]
    fn test_bolt_kills_bears() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(bears).end_turn(), p2.end_turn()])?;
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        let [bolt] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        Ok(())
    }

    #[test]
    fn test_murder_kills_bears() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MURDER]).mana_pool(mana![1, B, B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [murder] = game.player(p1).cards_in(HAND).exactly::<1>(MURDER)?;
        game.play([p1.cast(murder).targeting(bears).end_turn(), p2.end_turn()])?;
        assert_eq!(game.zone_of(bears), GRAVEYARD);
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
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        let [blade] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(DOOM_BLADE)?;
        assert_eq!(game.zone_of(blade), GRAVEYARD);
        Ok(())
    }
}
