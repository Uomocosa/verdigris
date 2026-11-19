use crate::actions;
use crate::testing;

pub fn target_choice(game: &testing::MockGame) -> Option<testing::ExpectedTargets> {
    game.app
        .world()
        .get_resource::<actions::PendingChoice>()?
        .as_ref()
        .map(|choice| testing::ExpectedTargets {
            count: choice.count,
            legal: choice.legal.clone(),
        })
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::target_choice;

    #[test]
    fn test_usage() {
        let game = testing::MockGame::builder().build();
        assert!(target_choice(&game).is_none());
    }
}
