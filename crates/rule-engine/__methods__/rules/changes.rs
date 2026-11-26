use bevy::prelude::Entity;

use crate::effects;
use crate::rules;

pub fn changes(
    answers: &rules::Rules,
    entity: Entity,
    target: rules::Target,
) -> Vec<rules::Change> {
    answers
        .mods(entity)
        .into_iter()
        .filter_map(|modification| match modification {
            effects::Modification::Rule(change) if change.target == target => Some(change.change),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::changes;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([TEST_RULE_WAIVER]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    changes(&answers, *waiver, rules::Target::Clause(UNTAPPED.key)),
                    vec![rules::Change::AsThough]
                );
                assert_eq!(
                    changes(&answers, *waiver, rules::Target::event::<TestStep>()),
                    vec![rules::Change::Skip]
                );
                assert_eq!(
                    changes(&answers, *waiver, rules::Target::event::<TestRule>()),
                    vec![rules::Change::Must]
                );
                assert_eq!(
                    changes(
                        &answers,
                        Entity::from(p1),
                        rules::Target::Clause(UNTAPPED.key)
                    ),
                    Vec::new()
                );
            })
            .unwrap();
        Ok(())
    }
}
