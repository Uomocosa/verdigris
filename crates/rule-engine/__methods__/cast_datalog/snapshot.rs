use crate::events;
use crate::players;
use crate::rules;
use frogmyr::Facts;

use crate::legality;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = legality::PlayDatalog::snapshot(answers, viewer);
    let players_in_game = answers.players_in_game(rules::Players::All, Some(viewer));
    for card in legality::PlayDatalog::visible_entities(answers, viewer) {
        for player in &players_in_game {
            let event = events::Cast {
                spell: card,
                caster: *player,
            };
            legality::PlayDatalog::pair_facts_for(
                answers,
                &mut facts,
                &event,
                card,
                **player,
                legality::CastDatalog::CAST,
                &rules::ArgRef::Arg(events::Cast::SPELL),
                &rules::ArgRef::Arg(events::Cast::CASTER),
            );
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::legality;
    use crate::players;
    use crate::rules;
    use crate::testing::FOREST;
    use crate::turns;
    use crate::zones;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use frogmyr::Value;

    use super::snapshot;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
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
        app.world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let facts = snapshot(&answers, first);
                assert!(facts.contains(
                    &legality::PlayDatalog::OBJECT,
                    &[Value::Entity(forest.to_bits())]
                ));
                assert!(facts.contains(
                    &legality::PlayDatalog::PRIORITY,
                    &[Value::Entity(first.to_bits())]
                ));
            })
            .unwrap();
    }
}
