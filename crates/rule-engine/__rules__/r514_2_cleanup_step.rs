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
            commands
                .entity(entity)
                .remove::<effects::PowerToughnessModifier>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::damage;
    use crate::effects;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([GIANT_GROWTH])
                .battlefield([GRIZZLY_BEARS])
                .mana_pool(mana![G]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let growth = game.player(p2).cards(HAND, GIANT_GROWTH).only().unwrap();
        let bears = game
            .player(p2)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();

        game.play([
            p1.cast(bolt).targeting(bears).pass_priority(),
            p2.cast(growth).targeting(bears).pass_priority(),
            p1.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();

        assert_eq!(game.zone_of(bears), BATTLEFIELD);
        assert_eq!(
            game.app.world().get::<damage::Damage>(bears).copied(),
            Some(damage::Damage(0))
        );
        assert!(
            game.app
                .world()
                .get::<effects::PowerToughnessModifier>(bears)
                .is_none()
        );
    }

    #[test]
    fn r514_2_until_end_of_turn_effect_expires_before_the_next_bolt() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GIANT_GROWTH, LIGHTNING_BOLT])
                .battlefield([GRIZZLY_BEARS])
                .mana_pool(mana![G, R]),
            P2,
        ]);
        let growth = game.player(p1).cards(HAND, GIANT_GROWTH).only().unwrap();
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let bears = game
            .player(p1)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();

        game.play([
            p1.cast(growth).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();
        game.play([
            p1.pass_priority(),
            p1.cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(bears), GRAVEYARD);
    }
}
