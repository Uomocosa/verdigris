use bevy::prelude::Entity;

use crate::BATTLEFIELD;
use crate::effects;
use crate::rules;

pub fn static_changes(answers: &rules::Rules, entity: Entity) -> Vec<rules::RuleChange> {
    let mut timed: Vec<(u64, rules::RuleChange)> = Vec::new();
    for (source, definition, in_zone) in answers.static_sources.iter() {
        if !answers.zones.contains(BATTLEFIELD, **in_zone) {
            continue;
        }
        let stamp = answers
            .entities
            .get(source)
            .ok()
            .and_then(|found| found.get::<effects::SourceTimestamp>().map(|stamp| **stamp))
            .unwrap_or(u64::MAX);
        for ability in definition.continuous_abilities() {
            let effects::Modification::Rule(change) = &ability.modification else {
                continue;
            };
            if ability.applies(answers, source, entity) {
                timed.push((stamp, change.clone()));
            }
        }
    }
    timed.sort_by_key(|(stamp, _)| *stamp);
    timed.into_iter().map(|(_, change)| change).collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::static_changes;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]),
            P2.battlefield([LEYLINE_OF_THE_VOID]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    static_changes(&answers, *bolt),
                    vec![rules::RuleChange {
                        target: rules::Target::event::<events::PutIntoGraveyard>(),
                        change: rules::Change::Instead {
                            holder: events::PutIntoGraveyard::OBJECT.into(),
                            when: rules::always(),
                            then: rules::Then::Events(vec![rules::EventTemplate::Exile {
                                what: events::PutIntoGraveyard::OBJECT.into(),
                            }]),
                        },
                    }]
                );
            })
            .unwrap();
        Ok(())
    }
}
