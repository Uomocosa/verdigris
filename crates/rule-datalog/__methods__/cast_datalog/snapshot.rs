use verdigris_datalog_ir::Facts;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;

use crate::CastDatalog;
use crate::PlayDatalog;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = PlayDatalog::snapshot(answers, viewer);
    let players_in_game = answers.players_in_game(rules::Players::All, Some(viewer));
    for card in PlayDatalog::visible_entities(answers, viewer) {
        for player in &players_in_game {
            let event = events::Cast {
                spell: card,
                caster: *player,
            };
            PlayDatalog::pair_facts_for(
                answers,
                &mut facts,
                &event,
                card,
                **player,
                CastDatalog::CAST,
            );
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use verdigris_datalog_ir::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::snapshot;
    use crate::PlayDatalog;

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
                assert!(facts.contains(&PlayDatalog::OBJECT, &[Value::Entity(forest.to_bits())]));
                assert!(facts.contains(&PlayDatalog::PRIORITY, &[Value::Entity(first.to_bits())]));
            })
            .unwrap();
    }
}
