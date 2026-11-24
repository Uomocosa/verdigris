use bevy::prelude::Entity;

use crate::abilities;
use crate::effects;
use crate::keywords;
use crate::rules;

pub fn mods(answers: &rules::Rules, entity: Entity) -> Vec<effects::Modification> {
    let mut modifications: Vec<effects::Modification> = answers
        .characteristics(entity)
        .keywords
        .into_iter()
        .flat_map(keywords::StaticKeyword::keyword_modifications)
        .collect();
    modifications.extend(
        answers
            .definitions
            .get(entity)
            .into_iter()
            .flat_map(|definition| &definition.text)
            .filter_map(|ability| match ability {
                abilities::Ability::Static(abilities::StaticAbility::Replacement(
                    abilities::Replacement::Rule(change),
                )) => Some(effects::Modification::Rule(change.clone())),
                _ => None,
            }),
    );
    let mut ordered: Vec<effects::ContinuousEffect> =
        answers.effects.iter().filter(|effect| effect.affected == entity).cloned().collect();
    ordered.sort_by_key(|effect| effect.timestamp);
    modifications.extend(ordered.into_iter().map(|effect| effect.modification));
    modifications
}

#[cfg(test)]
mod tests {
    use crate::rules::traits::event::Event;
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::effects;
    use crate::rules;
    use crate::testing::*;

    use super::mods;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([TEST_RULE_WAIVER]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let modifications = mods(&answers, *waiver);
                assert_eq!(
                    modifications,
                    [
                        effects::Modification::Rule(rules::RuleChange {
                            target: rules::Target::Clause(UNTAPPED.key),
                            change: rules::Change::AsThough,
                        }),
                        effects::Modification::Rule(rules::RuleChange {
                            target: rules::Target::Event(TestRule::ID),
                            change: rules::Change::Must,
                        }),
                        effects::Modification::Rule(rules::RuleChange {
                            target: rules::Target::Event(TestStep::ID),
                            change: rules::Change::Skip,
                        }),
                    ]
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn reach_grants_no_keyword_change() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([P1, P2.battlefield([GIANT_SPIDER])]);
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(
                    !mods(&answers, *spider).contains(&effects::Modification::Rule(
                        rules::RuleChange {
                            target: rules::Target::Clause(rules::declare_blocker::FLYING.key),
                            change: rules::Change::AsThough,
                        }
                    ))
                );
            })
            .unwrap();
        Ok(())
    }
}
