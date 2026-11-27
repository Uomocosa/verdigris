use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, Query};

use crate::damage;
use crate::effects;
use crate::turns;

/// 514.2. Second, the following actions happen simultaneously: all damage marked on permanents
/// (including phased-out permanents) is removed and all “until end of turn” and “this turn” effects
/// end. This turn-based action doesn’t use the stack.
pub fn r514_2_cleanup_step() -> ScheduleConfigs<ScheduleSystem> {
    clear_damage_and_end_effects.run_if(turns::Step::Cleanup.begins())
}

fn clear_damage_and_end_effects(
    mut commands: Commands,
    damaged: Query<(Entity, &damage::Damage)>,
    expiring: Query<(Entity, &effects::ContinuousEffect)>,
) {
    for (entity, _) in damaged.iter() {
        commands.entity(entity).insert(damage::Damage(0));
    }
    for (entity, effect) in expiring.iter() {
        if effect.until == effects::Duration::UntilEndOfTurn {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::damage;
    use crate::mana;
    use crate::testing;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND, MAIN_1};

    #[test]
    fn cleanup_removes_damage_and_until_end_of_turn_effects() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([GIANT_GROWTH]).battlefield([GRIZZLY_BEARS]).mana_pool(mana![G]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [growth] = game.player(p2).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p1.cast(bolt).targeting(bears).pass_priority(),
            p2.cast(growth).targeting(bears).pass_priority(),
            p1.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p1.end_turn(), p2.end_turn()])?;

        assert!(game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert_eq!(game.get::<damage::Damage>(bears), Some(damage::Damage(0)));
        assert_eq!(game.pt(bears), Some(crate::pt!(2, 2)));
        Ok(())
    }

    #[test]
    fn until_end_of_turn_effect_expires_before_the_next_bolt() -> testing::TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GIANT_GROWTH, LIGHTNING_BOLT]).battlefield([FOREST, MOUNTAIN, GRIZZLY_BEARS]),
            P2,
        ]);
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;

        game.play([
            p1.activate_ability(forest.ability()).cast(growth).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p1.pass_to(MAIN_1), p2.pass_to(MAIN_1)])?;

        game.play([
            p1.activate_ability(mountain.ability()).cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        Ok(())
    }
}
