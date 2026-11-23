use bevy::prelude::Entity;

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
    let mut ordered: Vec<effects::ContinuousEffect> =
        answers.effects.iter().filter(|effect| effect.affected == entity).cloned().collect();
    ordered.sort_by_key(|effect| effect.timestamp);
    modifications.extend(ordered.into_iter().map(|effect| effect.modification));
    modifications
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
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
                        effects::Modification::Rule(rules::RuleChange::AsThough(UNTAPPED.key)),
                        effects::Modification::Rule(rules::RuleChange::Requirement(TestRule::ID)),
                        effects::Modification::Skip(TEST_STEP),
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
                        rules::RuleChange::AsThough(rules::declare_blocker::FLYING.key)
                    ))
                );
            })
            .unwrap();
        Ok(())
    }
}
