use crate::BATTLEFIELD;
use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::cards;
use crate::damage;
use crate::rules::helpers;
use crate::zones;

/// 704.5h If a creature has toughness greater than 0, and it's been dealt damage by a source with
/// deathtouch since the last time state-based actions were checked, that creature is destroyed.
/// Regeneration can replace this event.
pub fn r704_5h_deathtouch_damage(
    mut destructions: MessageWriter<damage::Destruction>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &damage::DealtByDeathtouch,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for (creature, definition, _, in_zone) in cards.iter() {
        commands.entity(creature).remove::<damage::DealtByDeathtouch>();
        if !helpers::is_card_in_zone(**in_zone, BATTLEFIELD, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = definition.power_toughness else {
            continue;
        };
        if power_toughness.toughness > 0 {
            destructions.write(damage::Destruction(creature));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GRAVEYARD;
    use crate::actions;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![rats.into()]));
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.player(p2).life(), 19);
    }

    #[test]
    fn r704_5h_deathtouch_damage_destroys_a_larger_creature() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![rats.into()]));
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.act(
            p2,
            actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                blocker: bears.into(),
                attacker: rats.into(),
            }]),
        );
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(rats), GRAVEYARD);
        assert_eq!(game.zone_of(bears), GRAVEYARD);
    }

    #[test]
    fn r704_5h_deathtouch_damage_does_not_destroy_indestructible() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![rats.into()]));
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let [myr] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR).unwrap();
        game.act(
            p2,
            actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                blocker: myr.into(),
                attacker: rats.into(),
            }]),
        );
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(rats), BATTLEFIELD);
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
    }

    #[test]
    fn r704_5h_damage_without_deathtouch_does_not_destroy_a_two_two() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WALKING_CORPSE]),
            P2.battlefield([GRIZZLY_BEARS, FOREST]).hand([GIANT_GROWTH]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [corpse] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![corpse.into()]));
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.act(
            p2,
            actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                blocker: bears.into(),
                attacker: corpse.into(),
            }]),
        );
        let [growth] = game.player(p2).cards_in(HAND).exactly::<1>(GIANT_GROWTH).unwrap();
        let [forest] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FOREST).unwrap();
        game.act(
            p2,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: crate::cards::Color::Green,
            },
        );
        game.play([
            p2.cast(growth).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])
        .unwrap();
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        assert_eq!(game.zone_of(corpse), GRAVEYARD);
    }
}
