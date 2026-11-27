use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use std::collections::BTreeSet;

use crate::actions;
use crate::players;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickChoice {
    pub id: actions::ChoiceId,
    pub player: players::PlayerId,
    pub source: Option<Entity>,
    pub among: BTreeSet<Entity>,
    pub count: actions::Count,
    pub ordered: bool,
}

#[atomic_delegates]
impl PickChoice {
    #[must_use]
    pub fn check(&self, picked: &[Entity]) -> Result<(), actions::ChoiceWhy> {}
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: BTreeSet::from([first]),
            count: actions::Count::exactly(1),
            ordered: false,
        };
        assert_eq!(choice.check(&[first]), Ok(()));
        assert_eq!(
            choice.check(&[]),
            Err(actions::ChoiceWhy::TooFew { min: 1 })
        );
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
