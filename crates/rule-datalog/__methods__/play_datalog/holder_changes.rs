use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event_args::EventArgs;

pub fn holder_changes(
    answers: &rules::Rules,
    event: &events::Play,
    target: rules::Target,
) -> Vec<rules::Change> {
    let mut kept = Vec::new();
    for id in events::Play::args() {
        for reference in [
            rules::ArgRef::Arg(*id),
            rules::ArgRef::ControllerOf(*id),
            rules::ArgRef::OwnerOf(*id),
        ] {
            let Some(entity) = answers.resolve(reference, event).and_then(|value| match value {
                rules::ArgValue::Entity(found) => Some(found),
                rules::ArgValue::Player(found) => Some(*found),
                rules::ArgValue::Entities(_)
                | rules::ArgValue::Amount(_)
                | rules::ArgValue::Event(_) => None,
            }) else {
                continue;
            };
            for change in answers.changes(entity, target) {
                let held = match &change {
                    rules::Change::Restrict { holder, .. }
                    | rules::Change::AsThough { holder }
                    | rules::Change::Allow { holder, .. }
                    | rules::Change::Instead { holder, .. }
                    | rules::Change::Hook { holder, .. } => *holder == reference,
                    _ => false,
                };
                if held {
                    kept.push(change);
                }
            }
        }
    }
    kept
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

    use super::holder_changes;

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
                assert_eq!(
                    holder_changes(&answers, &event, rules::Target::event::<events::Play>()),
                    []
                );
            })
            .unwrap();
    }
}
