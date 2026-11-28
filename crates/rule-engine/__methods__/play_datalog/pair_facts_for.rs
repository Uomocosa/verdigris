use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Atom;
use frogmyr::Facts;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn pair_facts_for<E: Event>(
    answers: &rules::Rules,
    facts: &mut Facts,
    event: &E,
    card: Entity,
    holder: Entity,
    event_name: &'static str,
    card_ref: &rules::ArgRef,
    holder_ref: &rules::ArgRef,
) {
    let candidate = Atom {
        rel: legality::PlayDatalog::PLAY_CANDIDATE,
        terms: vec![
            Term::Var(legality::PlayDatalog::CARD_VAR),
            Term::Var(legality::PlayDatalog::PLAYER_VAR),
        ],
    };
    for restriction in E::restrictions() {
        if !restriction.unwaivable
            && legality::PlayDatalog::untranslated_waiver(
                answers,
                event,
                restriction.id,
                card_ref,
                holder_ref,
                &candidate,
            )
        {
            let _ = facts.add(
                legality::PlayDatalog::WAIVED,
                vec![
                    Value::Sym(event_name),
                    Value::Entity(card.to_bits()),
                    Value::Entity(holder.to_bits()),
                    Value::Sym(*restriction.id),
                ],
            );
        }
    }
    for change in legality::PlayDatalog::holder_changes(answers, event, rules::Target::event::<E>())
    {
        if let rules::Change::Restrict { restriction, .. } = change
            && !restriction.unwaivable
            && legality::PlayDatalog::untranslated_waiver(
                answers,
                event,
                restriction.id,
                card_ref,
                holder_ref,
                &candidate,
            )
        {
            let _ = facts.add(
                legality::PlayDatalog::WAIVED,
                vec![
                    Value::Sym(event_name),
                    Value::Entity(card.to_bits()),
                    Value::Entity(holder.to_bits()),
                    Value::Sym(*restriction.id),
                ],
            );
        }
    }
    for change in legality::PlayDatalog::holder_changes(answers, event, rules::Target::event::<E>())
    {
        if let rules::Change::Restrict { restriction, .. } = change
            && !legality::PlayDatalog::translatable_restriction(
                event,
                &restriction,
                card_ref,
                holder_ref,
                &candidate,
            )
            && !answers.eval(&restriction.condition.bind(event), event, None)
        {
            let _ = facts.add(
                legality::PlayDatalog::ADDED,
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
    use crate::cards;
    use crate::events;
    use crate::legality;
    use crate::players;
    use crate::rules;
    use crate::testing::FOREST;
    use crate::turns;
    use crate::zones;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use frogmyr::Facts;
    use frogmyr::Term;
    use frogmyr::Value;

    use super::pair_facts_for;

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
                    legality::PlayDatalog::PLAY,
                    &rules::ArgRef::Arg(events::Play::CARD),
                    &rules::ArgRef::Arg(events::Play::PLAYER),
                );
                let waived: Vec<&Vec<Value>> = facts.matching(
                    &legality::PlayDatalog::WAIVED,
                    &[
                        Term::Val(Value::Sym(legality::PlayDatalog::PLAY)),
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
