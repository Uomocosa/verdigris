use bevy::prelude::{Entity, MessageReader, MessageWriter, ParamSet, Query};

use crate::cards;
use crate::effects;
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
    mut damage_dealt: MessageWriter<zones::DamageDealt>,
    mut destructions: MessageWriter<zones::Destruction>,
    mut countered: MessageWriter<stack::Countered>,
    mut draws: MessageWriter<zones::DrawRequested>,
    mut player_effects: ParamSet<(
        MessageWriter<zones::LifeLost>,
        MessageWriter<zones::DiscardRequested>,
    )>,
    mut power_toughness_modified: MessageWriter<effects::PowerToughnessModified>,
    spells: Query<(
        &cards::CardDefinition,
        &zones::Owner,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    lives: Query<&zones::Life>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, owner, in_zone, chosen)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, &zone_entities) {
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
                        damage_dealt.write(zones::DamageDealt {
                            source: spell,
                            target,
                            amount,
                        });
                    }
                }
                effects::EffectKind::Destroy => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(
                            **target_zone,
                            zones::Zone::Battlefield,
                            &zone_entities,
                        )
                    {
                        destructions.write(zones::Destruction(target));
                    }
                }
                effects::EffectKind::Counter => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(
                            **target_zone,
                            zones::Zone::Stack,
                            &zone_entities,
                        )
                    {
                        countered.write(stack::Countered(target));
                    }
                }
                effects::EffectKind::Draw(count) => {
                    draws.write(zones::DrawRequested {
                        player: player_target.unwrap_or(**owner),
                        count,
                    });
                }
                effects::EffectKind::LoseLife(amount) => {
                    if let Some(player) = player_target {
                        player_effects
                            .p0()
                            .write(zones::LifeLost { player, amount });
                    }
                }
                effects::EffectKind::Discard(count) => {
                    if let Some(player) = player_target {
                        player_effects
                            .p1()
                            .write(zones::DiscardRequested { player, count });
                    }
                }
                effects::EffectKind::ModifyPowerToughness { power, toughness } => {
                    if let Some(target) = chosen_target
                        && let Ok((_, _, target_zone, _)) = spells.get(target)
                        && helpers::is_card_in_zone(
                            **target_zone,
                            zones::Zone::Battlefield,
                            &zone_entities,
                        )
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
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    fn cast_and_resolve(
        game: &mut testing::MockGame,
        spell_name: &str,
        target: Option<bevy::prelude::Entity>,
    ) {
        let spell = game.find(game.p1, zones::Zone::Hand, spell_name).unwrap();
        game.act(game.p1, actions::Action::Cast(spell));
        if let Some(target) = target {
            game.act(game.p1, actions::Action::ChooseTargets(vec![target]));
        }
        game.pass_priority_all();
    }

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let p2 = game.p2;
        cast_and_resolve(&mut game, "Lightning Bolt", Some(p2));
        assert_eq!(game.life(game.p2), 17);
    }

    #[test]
    fn test_bolt_kills_bears() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        cast_and_resolve(&mut game, "Lightning Bolt", Some(bears));
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert!(
            game.find(game.p2, zones::Zone::Graveyard, "Grizzly Bears")
                .is_some()
        );
        let bolt = game
            .find(game.p1, zones::Zone::Graveyard, "Lightning Bolt")
            .unwrap();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
    }

    #[test]
    fn test_bolt_to_face() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let p2 = game.p2;
        cast_and_resolve(&mut game, "Lightning Bolt", Some(p2));
        assert_eq!(game.life(game.p2), 17);
    }

    #[test]
    fn test_murder_kills_bears() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::murder()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![1, B, B])
            .build();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        cast_and_resolve(&mut game, "Murder", Some(bears));
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
    }

    #[test]
    fn doom_blade_destroys_a_green_creature() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::doom_blade()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![1, B])
            .build();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        cast_and_resolve(&mut game, "Doom Blade", Some(bears));
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert!(
            game.find(game.p2, zones::Zone::Graveyard, "Grizzly Bears")
                .is_some()
        );
        let blade = game
            .find(game.p1, zones::Zone::Graveyard, "Doom Blade")
            .unwrap();
        assert_eq!(game.zone_of(blade), zones::Zone::Graveyard);
    }
}
