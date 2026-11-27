use derive_more::Deref;

use crate::abilities;
use crate::rules;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct BeginningOf(pub turns::Step);

#[rustfmt::skip]
impl BeginningOf {
    #[must_use]
    pub fn of(self, players: impl Into<rules::Players>) -> abilities::TriggerEvent {
        abilities::TriggerEvent::BeginningOfStep(*self, players.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::rules;
    use crate::turns;

    use super::BeginningOf;

    #[test]
    fn test_usage() {
        let beginning = BeginningOf(turns::Step::Upkeep);
        assert_eq!(*beginning, turns::Step::Upkeep);
        assert_eq!(
            turns::beginning_of(UPKEEP).of(YOU),
            abilities::TriggerEvent::BeginningOfStep(turns::Step::Upkeep, rules::Players::You)
        );
    }
}
