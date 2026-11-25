use derive_more::Deref;

use crate::turns;

#[derive(Clone, Copy, Deref)]
pub struct TurnView<'w>(pub(crate) Option<&'w turns::TurnState>);

#[rustfmt::skip]
impl TurnView<'_> {
    #[must_use]
    pub fn step(self) -> Option<turns::Step> { self.map(|state| state.turn.step) }

    #[must_use]
    pub fn next_step(self) -> Option<turns::Step> { self.and_then(turns::TurnState::next_step) }

    #[must_use]
    pub fn is_first_turn(self) -> bool { self.is_some_and(|state| state.turn.number == 1) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::turns;

    use super::TurnView;

    #[test]
    fn test_usage() {
        let state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: players::PlayerId(Entity::PLACEHOLDER),
            step: turns::Step::PrecombatMain,
        });
        let view = TurnView(Some(&state));
        assert_eq!(view.step(), Some(turns::Step::PrecombatMain));
        assert_eq!(view.next_step(), Some(turns::Step::BeginningOfCombat));
        assert!(view.is_first_turn());
        let missing = TurnView(None);
        assert_eq!(missing.step(), None);
        assert_eq!(missing.next_step(), None);
        assert!(!missing.is_first_turn());
    }
}
