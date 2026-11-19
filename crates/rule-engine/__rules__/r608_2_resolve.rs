use bevy::prelude::{Entity, MessageReader, MessageWriter, Query};

use crate::cards;
use crate::effects;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 608.2 If the object that's resolving is an instant spell, a sorcery spell, or an ability, its
/// resolution may involve several steps. ... The steps described in rule 608.2n are followed
/// last.
pub fn r608_2_resolve(
    mut resolutions: MessageReader<stack::Resolution>,
    mut damage_dealt: MessageWriter<zones::DamageDealt>,
    mut destructions: MessageWriter<zones::Destruction>,
    spells: Query<(
        &cards::CardDefinition,
        &zones::InZone,
        Option<&stack::ChosenTarget>,
    )>,
    targets: Query<&zones::InZone>,
    lives: Query<&zones::Life>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((definition, in_zone, chosen)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, &zone_entities) {
            continue;
        }
        let Some(effect) = definition.effect else {
            continue;
        };
        let Some(target) = chosen.map(|chosen| **chosen) else {
            continue;
        };
        match effect.kind {
            effects::EffectKind::Damage(amount) => {
                if lives.get(target).is_ok() || targets.get(target).is_ok() {
                    damage_dealt.write(zones::DamageDealt {
                        source: spell,
                        target,
                        amount,
                    });
                }
            }
            effects::EffectKind::Destroy => {
                if let Ok(target_zone) = targets.get(target)
                    && helpers::is_card_in_zone(
                        **target_zone,
                        zones::Zone::Battlefield,
                        &zone_entities,
                    )
                {
                    destructions.write(zones::Destruction(target));
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
