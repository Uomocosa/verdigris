use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use std::collections::BTreeSet;

use crate::actions;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickChoice {
    pub id: actions::ChoiceId,
    pub player: players::PlayerId,
    pub source: Option<Entity>,
    pub among: actions::Candidates,
    pub count: actions::Count,
    pub ordered: bool,
}

#[atomic_delegates]
impl PickChoice {
    #[must_use]
    pub fn candidates(&self, answers: &rules::Rules) -> Vec<(Entity, rules::Legality)> {}

    #[must_use]
    pub fn check(
        &self,
        answers: &rules::Rules,
        picked: &[Entity],
    ) -> Result<(), actions::ChoiceWhy> {
    }
}

impl PickChoice {
    #[must_use]
    pub const fn fixed(&self) -> Option<&BTreeSet<Entity>> {
        match &self.among {
            actions::Candidates::Fixed(set) => Some(set),
            actions::Candidates::Matching { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use std::collections::BTreeSet;

    use crate::HAND;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, HAND).unwrap();
        let choice = actions::PickChoice {
            id: actions::ChoiceId("701.9b/discard"),
            player: p1.id,
            source: None,
            among: actions::Candidates::Fixed(BTreeSet::from([first])),
            count: actions::Count::exactly(1),
            ordered: false,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(choice.check(&answers, &[first]), Ok(()));
                assert_eq!(
                    choice.check(&answers, &[]),
                    Err(actions::ChoiceWhy::TooFew { min: 1 })
                );
            })
            .unwrap();
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
