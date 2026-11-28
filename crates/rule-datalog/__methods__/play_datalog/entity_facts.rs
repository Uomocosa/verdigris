use bevy::prelude::Entity;

use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Value;
use verdigris_rule_engine::cards;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::zones;

use crate::PlayDatalog;

pub fn entity_facts(answers: &rules::Rules, facts: &mut Facts, entity: Entity) {
    let zone_name = |zone: zones::Zone| match zone {
        zones::Zone::Library => "Library",
        zones::Zone::Hand => "Hand",
        zones::Zone::Battlefield => "Battlefield",
        zones::Zone::Graveyard => "Graveyard",
        zones::Zone::Exile => "Exile",
        zones::Zone::Stack => "Stack",
    };
    let _ = facts.add(PlayDatalog::OBJECT, vec![Value::Entity(entity.to_bits())]);
    if let Ok(definition) = answers.definitions.get(entity) {
        for card_type in &definition.type_line.types {
            let _ = facts.add(
                PlayDatalog::CARD_TYPE,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym(PlayDatalog::card_type_name(*card_type)),
                ],
            );
        }
    }
    let zone_lookup = answers
        .entities
        .get(entity)
        .ok()
        .and_then(|found| found.get::<zones::InZone>().copied())
        .and_then(|in_zone| answers.zones.get(*in_zone).ok());
    if let Some((_, zone, owner)) = zone_lookup {
        if *zone == zones::Zone::Stack {
            let _ = facts.add(PlayDatalog::ON_STACK, vec![Value::Entity(entity.to_bits())]);
        }
        if let Some(owner) = owner {
            let _ = facts.add(
                PlayDatalog::IN_ZONE,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym(zone_name(*zone)),
                    Value::Entity((***owner).to_bits()),
                ],
            );
        }
    }
    if let Some(owner) =
        answers.entities.get(entity).ok().and_then(|found| found.get::<cards::Owner>())
    {
        let _ = facts.add(
            PlayDatalog::OWNER,
            vec![
                Value::Entity(entity.to_bits()),
                Value::Entity((***owner).to_bits()),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::App;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::zones;

    use super::entity_facts;
    use crate::PlayDatalog;

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
                let mut facts = Facts::default();
                entity_facts(&answers, &mut facts, forest);
                assert!(facts.contains(&PlayDatalog::OBJECT, &[Value::Entity(forest.to_bits())]));
                assert!(facts.contains(
                    &PlayDatalog::CARD_TYPE,
                    &[Value::Entity(forest.to_bits()), Value::Sym("Land")]
                ));
            })
            .unwrap();
    }
}
