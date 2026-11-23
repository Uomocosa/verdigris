use bevy::prelude::Entity;

use crate::effects;
use crate::keywords;
use crate::rules;

pub fn mods(answers: &rules::Rules, entity: Entity) -> Vec<effects::Change> {
    let mut changes: Vec<effects::Change> = answers
        .effects
        .iter()
        .filter(|effect| effect.affected == entity)
        .map(|effect| effect.change.clone())
        .collect();
    changes.extend(
        answers
            .characteristics(entity)
            .keywords
            .into_iter()
            .flat_map(keywords::StaticKeyword::keyword_changes),
    );
    changes
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
                let changes = mods(&answers, *waiver);
                assert_eq!(
                    changes,
                    [
                        effects::Change::Waive(UNTAPPED.key),
                        effects::Change::Demand(TestRule::ID),
                        effects::Change::Skip(TEST_STEP),
                    ]
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn keywords_grant_their_changes() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([P1, P2.battlefield([GIANT_SPIDER])]);
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(
                    mods(&answers, *spider).contains(&effects::Change::Waive(rules::FLYING.key))
                );
            })
            .unwrap();
        Ok(())
    }
}
