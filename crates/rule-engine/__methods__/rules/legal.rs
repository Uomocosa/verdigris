use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::rules::traits::pair_ctx::PairCtx;

pub fn legal<R: LegalityRule>(answers: &rules::Rules, context: &R::Ctx) -> bool {
    let subject = context.subject();
    let other = context.other();
    let waived = |clause: &rules::Clause| {
        let entity = match clause.role {
            rules::Role::Subject => Some(subject),
            rules::Role::Other => other,
        };
        entity.is_some_and(|entity| {
            answers
                .mods(entity)
                .any(|change| matches!(change, effects::Change::Waive(key) if *key == clause.key))
        })
    };
    let base = R::clauses()
        .iter()
        .filter(|clause| !waived(clause))
        .all(|clause| answers.eval(&clause.requires, subject, other));
    let restricted = answers.mods(subject).any(|change| {
        matches!(change, effects::Change::Restrict(id, requirement) if *id == R::ID && !answers.eval(requirement, subject, other))
    });
    base && !restricted
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::legal;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(legal::<TestRule>(&answers, &rules::AttackCtx(*bears)));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_tapped_creature_fails_an_untapped_clause() -> TestResult {
        let (mut game, [_, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!legal::<TestRule>(&answers, &rules::AttackCtx(*bears)));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_waived_clause_is_skipped_for_the_waiving_creature() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield_tapped([TEST_RULE_WAIVER]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(legal::<TestRule>(&answers, &rules::AttackCtx(*waiver)));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_restriction_that_does_not_hold_is_illegal() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([TEST_RULE_RESTRICTION]), P2]);
        let [restriction] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_RESTRICTION)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!legal::<RestrictedRule>(
                    &answers,
                    &rules::AttackCtx(*restriction)
                ));
            })
            .unwrap();
        Ok(())
    }
}
