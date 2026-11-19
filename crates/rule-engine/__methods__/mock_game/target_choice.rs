use crate::actions;
use crate::testing;

pub fn target_choice(game: &testing::MockGame) -> Option<testing::ExpectedTargets> {
    let pending = game
        .app
        .world()
        .get_resource::<actions::PendingChoice>()?
        .as_ref()?;
    match pending {
        actions::Choice::Target(choice) => Some(testing::ExpectedTargets {
            count: choice.count,
            legal: choice.legal.clone(),
        }),
        actions::Choice::Discard(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::target_choice;

    #[test]
    fn test_usage() {
        let game = MockGame::builder().build();
        assert!(target_choice(&game).is_none());
    }
}
