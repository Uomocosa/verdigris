use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Value;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::turns;
use verdigris_rule_engine::zones;

use crate::PlayDatalog;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = Facts::default();
    let step_name = |step: turns::Step| match step {
        turns::Step::Untap => "Untap",
        turns::Step::Upkeep => "Upkeep",
        turns::Step::Draw => "Draw",
        turns::Step::PrecombatMain => "PrecombatMain",
        turns::Step::BeginningOfCombat => "BeginningOfCombat",
        turns::Step::DeclareAttackers => "DeclareAttackers",
        turns::Step::DeclareBlockers => "DeclareBlockers",
        turns::Step::CombatDamage => "CombatDamage",
        turns::Step::SecondCombatDamage => "SecondCombatDamage",
        turns::Step::EndOfCombat => "EndOfCombat",
        turns::Step::PostcombatMain => "PostcombatMain",
        turns::Step::End => "End",
        turns::Step::Cleanup => "Cleanup",
    };
    let players_in_game = answers.players_in_game(rules::Players::All, Some(viewer));
    for player in &players_in_game {
        let _ = facts.add(
            PlayDatalog::PLAYER_FACT,
            vec![Value::Entity(player.to_bits())],
        );
    }
    let visible = PlayDatalog::visible_entities(answers, viewer);
    for entity in &visible {
        PlayDatalog::entity_facts(answers, &mut facts, *entity);
    }
    if let Ok(state) = answers.state.single() {
        if let Some(holder) = state.priority.holder {
            let _ = facts.add(PlayDatalog::PRIORITY, vec![Value::Entity(holder.to_bits())]);
        }
        let _ = facts.add(
            PlayDatalog::ACTIVE,
            vec![Value::Entity(state.turn.active.to_bits())],
        );
        let _ = facts.add(
            PlayDatalog::STEP,
            vec![Value::Sym(step_name(state.turn.step))],
        );
    }
    let _ = facts.add(PlayDatalog::MAIN_STEP, vec![Value::Sym("PrecombatMain")]);
    let _ = facts.add(PlayDatalog::MAIN_STEP, vec![Value::Sym("PostcombatMain")]);
    if answers
        .zones
        .find(zones::SharedZone::Stack, None)
        .is_none_or(|stack| !answers.in_zone.iter().any(|in_zone| **in_zone == stack))
    {
        for player in &players_in_game {
            let _ = facts.add(
                PlayDatalog::STACK_EMPTY,
                vec![Value::Entity(player.to_bits())],
            );
        }
    }
    for entity in &visible {
        for player in &players_in_game {
            PlayDatalog::pair_facts(answers, &mut facts, *entity, *player);
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
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
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
        let battlefield =
            app.world_mut().spawn((zones::Zone::Battlefield, cards::Owner(first))).id();
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
        app.world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let facts = snapshot(&answers, first);
                assert!(facts.contains(&PlayDatalog::OBJECT, &[Value::Entity(forest.to_bits())]));
                assert!(facts.contains(
                    &PlayDatalog::CARD_TYPE,
                    &[Value::Entity(forest.to_bits()), Value::Sym("Land")]
                ));
                assert!(!facts.contains(
                    &PlayDatalog::CARD_TYPE,
                    &[Value::Entity(bears.to_bits()), Value::Sym("Land")]
                ));
                assert!(
                    facts.contains(&PlayDatalog::PLAYER_FACT, &[Value::Entity(first.to_bits())])
                );
                assert!(facts.contains(&PlayDatalog::PRIORITY, &[Value::Entity(first.to_bits())]));
                assert!(facts.contains(&PlayDatalog::ACTIVE, &[Value::Entity(first.to_bits())]));
                assert!(
                    facts.contains(&PlayDatalog::STACK_EMPTY, &[Value::Entity(first.to_bits())])
                );
            })
            .unwrap();
    }
}
