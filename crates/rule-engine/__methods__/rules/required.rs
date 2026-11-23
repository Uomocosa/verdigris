use bevy::prelude::Entity;

use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub fn required<R: LegalityRule>(answers: &rules::Rules, entity: Entity) -> bool {
    answers
        .mods(entity)
        .iter()
        .any(|modification| matches!(modification, effects::Modification::Rule(rules::RuleChange::Requirement(id)) if *id == R::ID))
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::required;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([TEST_RULE_WAIVER, GRIZZLY_BEARS]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(required::<TestRule>(&answers, *waiver));
                assert!(!required::<TestRule>(&answers, *bears));
            })
            .unwrap();
        Ok(())
    }
}
