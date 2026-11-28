use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::zones;

pub fn visible_entities(answers: &rules::Rules, viewer: players::PlayerId) -> Vec<Entity> {
    let mut entities: Vec<Entity> = answers
        .entities
        .iter()
        .filter_map(|entity| {
            let in_zone = entity.get::<zones::InZone>()?;
            let (_, zone, owner) = answers.zones.get(**in_zone).ok()?;
            if matches!(
                *zone,
                zones::Zone::Battlefield
                    | zones::Zone::Graveyard
                    | zones::Zone::Exile
                    | zones::Zone::Stack
            ) || *zone == zones::Zone::Hand && owner.is_some_and(|holder| ***holder == *viewer)
            {
                Some(entity.id())
            } else {
                None
            }
        })
        .collect();
    entities.sort();
    entities
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::players;
    use crate::rules;
    use crate::testing::FOREST;
    use crate::testing::GRIZZLY_BEARS;
    use crate::turns;
    use crate::zones;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;

    use super::visible_entities;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let second = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
        let battlefield =
            app.world_mut().spawn((zones::Zone::Battlefield, cards::Owner(first))).id();
        let other_hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(second))).id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: first,
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(first);
        app.insert_resource(state);
        let forest = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(hand),
                cards::Owner(first),
            ))
            .id();
        let bears = app
            .world_mut()
            .spawn((
                (GRIZZLY_BEARS.definition)(),
                zones::InZone(battlefield),
                cards::Owner(first),
            ))
            .id();
        let stranger = app
            .world_mut()
            .spawn((
                (FOREST.definition)(),
                zones::InZone(other_hand),
                cards::Owner(second),
            ))
            .id();
        app.world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = visible_entities(&answers, first);
                assert!(listed.contains(&forest));
                assert!(listed.contains(&bears));
                assert!(!listed.contains(&stranger));
            })
            .unwrap();
    }
}
