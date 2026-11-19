use crate::cards;
use crate::testing;

#[derive(Clone, Copy, Debug)]
pub enum CardQuery {
    Named(testing::Card),
    Exact(testing::TestCard),
}

#[rustfmt::skip]
impl From<testing::Card> for CardQuery {
    fn from(card: testing::Card) -> Self { Self::Named(card) }
}

#[rustfmt::skip]
impl From<testing::TestCard> for CardQuery {
    fn from(card: testing::TestCard) -> Self { Self::Exact(card) }
}

impl CardQuery {
    pub(crate) fn label(self) -> String {
        match self {
            Self::Named(query) => query.name.to_string(),
            Self::Exact(exact) => format!("{exact:?}"),
        }
    }

    pub(crate) fn matches(self, card: testing::TestCard, name: &cards::CardName) -> bool {
        match self {
            Self::Named(query) => name.as_str() == *query.name,
            Self::Exact(exact) => card == exact,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::testing;

    use super::CardQuery;

    #[test]
    fn test_usage() {
        let query = CardQuery::from(testing::LIGHTNING_BOLT);
        let name = cards::CardName("Lightning Bolt".to_string());

        assert_eq!(query.label(), "Lightning Bolt");
        assert!(query.matches(testing::TestCard(bevy::prelude::Entity::PLACEHOLDER), &name));
    }
}
