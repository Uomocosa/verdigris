use bevy::prelude::Entity;

use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Value;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;

pub fn pair_facts_for<E: Event>(
    answers: &rules::Rules,
    facts: &mut Facts,
    event: &E,
    card: Entity,
    holder: Entity,
    event_name: &'static str,
) {
    for restriction in E::restrictions() {
        if !restriction.unwaivable
            && PlayDatalog::waived(answers, event, restriction.id, restriction.unwaivable)
        {
            let _ = facts.add(
                PlayDatalog::WAIVED,
                vec![
                    Value::Sym(event_name),
                    Value::Entity(card.to_bits()),
                    Value::Entity(holder.to_bits()),
                    Value::Sym(*restriction.id),
                ],
            );
        }
    }
    for change in PlayDatalog::holder_changes(answers, event, rules::Target::event::<E>()) {
        if let rules::Change::Restrict { restriction, .. } = change
            && !PlayDatalog::waived(answers, event, restriction.id, restriction.unwaivable)
            && !answers.eval(&restriction.condition.bind(event), event, None)
        {
            let _ = facts.add(
                PlayDatalog::ADDED,
                vec![
                    Value::Entity(card.to_bits()),
                    Value::Entity(holder.to_bits()),
                    Value::Sym(*restriction.id),
                ],
            );
        }
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
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::pair_facts_for;
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
                let event = events::Play {
                    card: forest,
                    player: first,
                };
                pair_facts_for(
                    &answers,
                    &mut facts,
                    &event,
                    forest,
                    *first,
                    PlayDatalog::PLAY,
                );
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
