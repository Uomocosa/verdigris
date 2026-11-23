use crate::actions;
use crate::testing;

pub fn target_choice(game: &testing::MockGame) -> Option<testing::ExpectedTargets> {
    let pending = game.app.world().get_resource::<actions::PendingChoice>()?.as_ref()?;
    match pending {
        actions::Choice::Target(choice) => Some(testing::ExpectedTargets {
            count: choice.count,
            legal: choice.legal.clone(),
        }),
        actions::Choice::Discard(_)
        | actions::Choice::CombatDamage(_)
        | actions::Choice::OrderTriggers(_)
        | actions::Choice::ArrangeGraveyard(_)
        | actions::Choice::Legendary(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::target_choice;

    #[test]
    fn test_usage() {
        let (game, _) = MockGame::new([P1, P2]);
        assert!(target_choice(&game).is_none());
    }
}
