use bevy::prelude::{Commands, Entity, MessageReader, Query, With};

use crate::damage;
use crate::effects;
use crate::turns;

/// 514.2 Second, the following actions happen simultaneously: all damage marked on permanents
/// (including phased-out permanents) is removed and all "until end of turn" and "this turn"
/// effects end. This turn-based action doesn't use the stack.
pub fn r514_2_cleanup_step(
    mut step_started: MessageReader<turns::StepStarted>,
    mut commands: Commands,
    damaged: Query<(Entity, &damage::Damage)>,
    modified: Query<Entity, With<effects::PowerToughnessModifier>>,
) {
    for step in step_started.read() {
        if **step != turns::Step::Cleanup {
            continue;
        }
        for (entity, _) in damaged.iter() {
            commands.entity(entity).insert(damage::Damage(0));
        }
        for entity in modified.iter() {
            commands.entity(entity).remove::<effects::PowerToughnessModifier>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::damage;
    use crate::effects;
    use crate::mana;
    use crate::testing;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND, MAIN_1};

    #[test]
    fn test_usage() -> TestResult {
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

        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        assert_eq!(
            game.app.world().get::<damage::Damage>(bears.into()).copied(),
            Some(damage::Damage(0))
        );
        assert!(game.app.world().get::<effects::PowerToughnessModifier>(bears.into()).is_none());
        Ok(())
    }

    #[test]
    fn r514_2_until_end_of_turn_effect_expires_before_the_next_bolt() -> testing::TestResult {
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

        assert_eq!(game.zone_of(bears), GRAVEYARD);
        Ok(())
    }
}
