use bevy::prelude::Entity;

use crate::effects;
use crate::rules;

pub fn performs(answers: &rules::Rules, entity: Entity, step: rules::StepId) -> bool {
    let mut performs = true;
    for change in answers.mods(entity) {
        if let effects::Change::Skip(skipped) = change
            && skipped == step
        {
            performs = false;
        }
        if let effects::Change::Perform(performed) = change
            && performed == step
        {
            performs = true;
        }
    }
    performs
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::performs;

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
                assert!(!performs(&answers, *waiver, TEST_STEP));
                assert!(performs(&answers, *bears, TEST_STEP));
            })
            .unwrap();
        Ok(())
    }
}
