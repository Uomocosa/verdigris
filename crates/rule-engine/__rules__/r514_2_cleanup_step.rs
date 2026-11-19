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
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt()])
            .hand(Seat::P2, [giant_growth()])
            .battlefield(Seat::P2, [grizzly_bears()])
            .mana_pool(Seat::P1, mana![R])
            .mana_pool(Seat::P2, mana![G])
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
        let mut game = MockGame::builder()
            .hand(Seat::P1, [giant_growth(), lightning_bolt()])
            .battlefield(Seat::P1, [grizzly_bears()])
            .mana_pool(Seat::P1, mana![G, R])
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
