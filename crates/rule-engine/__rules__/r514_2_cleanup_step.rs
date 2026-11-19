use bevy::prelude::{Commands, Entity, MessageReader, Query, With};

use crate::effects;
use crate::turns;
use crate::zones;

/// 514.2 Second, the following actions happen simultaneously: all damage marked on permanents
/// (including phased-out permanents) is removed and all "until end of turn" and "this turn"
/// effects end. This turn-based action doesn't use the stack.
pub fn r514_2_cleanup_step(
    mut step_started: MessageReader<turns::StepStarted>,
    mut commands: Commands,
    damaged: Query<(Entity, &zones::Damage)>,
    modified: Query<Entity, With<effects::PowerToughnessModifier>>,
) {
    for step in step_started.read() {
        if **step != turns::Step::Cleanup {
            continue;
        }
        for (entity, _) in damaged.iter() {
            commands.entity(entity).insert(zones::Damage(0));
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
    use crate::actions;
    use crate::effects;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::grizzly_bears()])
            .build();
        let bears = game
            .find(game.p1, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        game.app.world_mut().entity_mut(bears).insert((
            zones::Damage(2),
            effects::PowerToughnessModifier {
                power: 3,
                toughness: 3,
            },
        ));
        game.end_turn();
        assert_eq!(
            game.app.world().get::<zones::Damage>(bears).copied(),
            Some(zones::Damage(0))
        );
        assert!(
            game.app
                .world()
                .get::<effects::PowerToughnessModifier>(bears)
                .is_none()
        );
    }

    #[test]
    fn r514_2_cleanup_removes_damage_and_ends_until_end_of_turn_effects() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .hand(testing::Seat::P2, [testing::giant_growth()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![R])
            .mana_pool(testing::Seat::P2, mana![G])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let growth = game
            .find(game.p2, zones::Zone::Hand, "Giant Growth")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        let p2 = game.p2;

        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(growth));
        game.act(p2, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();
        game.pass_priority_all();
        game.end_turn();

        assert_eq!(game.zone_of(bears), zones::Zone::Battlefield);
        assert_eq!(
            game.app.world().get::<zones::Damage>(bears).copied(),
            Some(zones::Damage(0))
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
        let mut game = testing::MockGame::builder()
            .hand(
                testing::Seat::P1,
                [testing::giant_growth(), testing::lightning_bolt()],
            )
            .battlefield(testing::Seat::P1, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![G, R])
            .build();
        let growth = game
            .find(game.p1, zones::Zone::Hand, "Giant Growth")
            .unwrap();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let bears = game
            .find(game.p1, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();

        game.act(game.p1, actions::Action::Cast(growth));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();
        game.end_turn();
        game.end_turn();
        game.pass_priority();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();

        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
    }
}
