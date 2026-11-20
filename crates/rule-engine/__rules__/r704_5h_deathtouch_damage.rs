use crate::BATTLEFIELD;
use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::cards;
use crate::damage;
use crate::zones;

/// 704.5h If a creature has toughness greater than 0, and it’s been dealt damage by a source with
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
        if !BATTLEFIELD.contains_card(**in_zone, &zone_entities) {
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
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).life(), 19);
        Ok(())
    }

    #[test]
    fn r704_5h_deathtouch_damage_destroys_a_larger_creature() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, rats)])])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.zone_of(rats), GRAVEYARD);
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        Ok(())
    }

    #[test]
    fn r704_5h_deathtouch_damage_does_not_destroy_indestructible() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [myr] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(myr, rats)])])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.zone_of(rats), BATTLEFIELD);
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        Ok(())
    }

    #[test]
    fn r704_5h_damage_without_deathtouch_does_not_destroy_a_two_two() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WALKING_CORPSE]),
            P2.battlefield([GRIZZLY_BEARS, FOREST]).hand([GIANT_GROWTH]),
        ]);
        let [corpse] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        let [growth] = game.player(p2).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([corpse]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, corpse)])])?;
        game.play([
            p2.activate_ability(forest.ability()).cast(growth).targeting(bears).pass_priority(),
            p1.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        assert_eq!(game.zone_of(corpse), GRAVEYARD);
        Ok(())
    }
}
