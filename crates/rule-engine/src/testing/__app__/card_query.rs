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
    pub(crate) fn matches(self, object: &testing::ZoneObject) -> Option<testing::TestCard> {
        let testing::ZoneObject::Card { card, name } = object else {
            return None;
        };
        let found = match self {
            Self::Named(query) => name.as_str() == *query.name,
            Self::Exact(exact) => *card == exact,
        };
        found.then_some(*card)
    }

    pub(crate) fn label(self) -> String {
        match self {
            Self::Named(query) => query.name.to_string(),
            Self::Exact(exact) => format!("{exact:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::cards;
    use crate::testing;

    use super::CardQuery;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let object = testing::ZoneObject::Card {
            card: testing::TestCard(entity),
            name: cards::CardName("Lightning Bolt".to_string()),
        };
        let query = CardQuery::from(testing::LIGHTNING_BOLT);

        assert_eq!(query.label(), "Lightning Bolt");
        assert_eq!(query.matches(&object), Some(testing::TestCard(entity)));
        assert_eq!(CardQuery::from(testing::MURDER).matches(&object), None);
    }
}
