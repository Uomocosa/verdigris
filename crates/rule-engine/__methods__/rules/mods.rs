use bevy::prelude::Entity;

use crate::effects;
use crate::rules;

pub fn mods<'a>(
    answers: &'a rules::Rules<'_, '_>,
    entity: Entity,
) -> impl Iterator<Item = &'a effects::Change> {
    answers
        .effects
        .iter()
        .filter(move |effect| effect.affected == entity)
        .map(|effect| &effect.change)
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
                let changes = mods(&answers, *waiver).collect::<Vec<_>>();
                assert_eq!(
                    changes,
                    [
                        &effects::Change::Waive(UNTAPPED.key),
                        &effects::Change::Demand(TestRule::ID),
                        &effects::Change::Skip(TEST_STEP),
                    ]
                );
            })
            .unwrap();
        Ok(())
    }
}
