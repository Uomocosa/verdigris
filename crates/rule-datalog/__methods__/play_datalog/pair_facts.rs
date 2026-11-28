use bevy::prelude::Entity;

use verdigris_datalog_ir::Facts;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;

use crate::PlayDatalog;

pub fn pair_facts(
    answers: &rules::Rules,
    facts: &mut Facts,
    card: Entity,
    player: players::PlayerId,
) {
    let event = events::Play { card, player };
    PlayDatalog::pair_facts_for(answers, facts, &event, card, player, PlayDatalog::PLAY);
    if answers.eval(
        &rules::land_play_available(rules::ArgRef::Arg(events::Play::PLAYER)),
        &event,
        None,
    ) {
        use verdigris_datalog_ir::Value;

        let _ = facts.add(
            PlayDatalog::LAND_PLAY_AVAILABLE,
            vec![Value::Entity(player.to_bits())],
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::pair_facts;
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
                let mut facts = Facts::default();
                pair_facts(&answers, &mut facts, forest, first);
                assert!(facts.contains(
                    &PlayDatalog::LAND_PLAY_AVAILABLE,
                    &[Value::Entity(first.to_bits())]
                ));
                let waived: Vec<&Vec<Value>> = facts.matching(
                    &PlayDatalog::WAIVED,
                    &[
                        Term::Val(Value::Sym(PlayDatalog::PLAY)),
                        Term::Val(Value::Entity(forest.to_bits())),
                        Term::Val(Value::Entity(first.to_bits())),
                        Term::Var("I"),
                    ],
                );
                let expected: Vec<&Vec<Value>> = Vec::new();
                assert_eq!(waived, expected);
            })
            .unwrap();
    }
}
