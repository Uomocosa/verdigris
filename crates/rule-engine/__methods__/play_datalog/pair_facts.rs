use bevy::prelude::Entity;

use crate::events;
use crate::players;
use crate::rules;
use frogmyr::Facts;

use crate::legality;

pub fn pair_facts(
    answers: &rules::Rules,
    facts: &mut Facts,
    card: Entity,
    player: players::PlayerId,
) {
    let event = events::Play { card, player };
    legality::PlayDatalog::pair_facts_for(
        answers,
        facts,
        &event,
        card,
        *player,
        legality::PlayDatalog::PLAY,
        &rules::ArgRef::Arg(events::Play::CARD),
        &rules::ArgRef::Arg(events::Play::PLAYER),
    );
    if answers.eval(
        &rules::land_play_available(rules::ArgRef::Arg(events::Play::PLAYER)),
        &event,
        None,
    ) {
        use frogmyr::Value;

        let _ = facts.add(
            legality::PlayDatalog::LAND_PLAY_AVAILABLE,
            vec![Value::Entity(player.to_bits())],
        );
    }
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
    use frogmyr::Facts;
    use frogmyr::Term;
    use frogmyr::Value;

    use super::pair_facts;

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
                    &legality::PlayDatalog::LAND_PLAY_AVAILABLE,
                    &[Value::Entity(first.to_bits())]
                ));
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
