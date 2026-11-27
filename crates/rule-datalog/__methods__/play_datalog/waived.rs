use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;

use crate::PlayDatalog;

pub fn waived(
    answers: &rules::Rules,
    event: &events::Play,
    id: rules::RestrictionId,
    unwaivable: bool,
) -> bool {
    if unwaivable {
        return false;
    }
    PlayDatalog::holder_changes(answers, event, rules::Target::Restriction(id)).iter().any(
        |change| match change {
            rules::Change::AsThough { .. } => true,
            rules::Change::Allow { when, .. } => answers.eval(&when.bind(event), event, None),
            _ => false,
        },
    )
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::zones;

    use super::waived;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let first = players::PlayerId(
            app.world_mut().spawn((players::Player, players::LandsPlayedThisTurn::NONE)).id(),
        );
        let hand = app.world_mut().spawn((zones::Zone::Hand, cards::Owner(first))).id();
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
                let event = events::Play {
                    card: forest,
                    player: first,
                };
                assert!(!waived(&answers, &event, events::play::FROM_HAND, false));
                assert!(!waived(&answers, &event, events::play::FROM_HAND, true));
            })
            .unwrap();
    }
}
