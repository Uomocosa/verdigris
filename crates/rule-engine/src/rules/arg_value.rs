use bevy::prelude::Entity;
use derive_more::From;

use crate::cards;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum ArgValue {
    Entity(Entity),
    Entities(Vec<Entity>),
    Player(players::PlayerId),
    Amount(u32),
    Event(rules::EventId),
}

#[rustfmt::skip]
impl From<cards::CardId> for ArgValue {
    fn from(card: cards::CardId) -> Self { Self::Entity(*card) }
}

impl ArgValue {
    #[must_use]
    pub const fn entity(&self) -> Option<Entity> {
        match self {
            Self::Entity(entity) => Some(*entity),
            _ => None,
        }
    }

    #[must_use]
    pub fn entities(&self) -> Option<Vec<Entity>> {
        match self {
            Self::Entities(entities) => Some(entities.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub const fn player(&self) -> Option<players::PlayerId> {
        match self {
            Self::Player(player) => Some(*player),
            _ => None,
        }
    }

    #[must_use]
    pub const fn amount(&self) -> Option<u32> {
        match self {
            Self::Amount(amount) => Some(*amount),
            _ => None,
        }
    }

    #[must_use]
    pub const fn event(&self) -> Option<rules::EventId> {
        match self {
            Self::Event(event) => Some(*event),
            _ => None,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> rules::ArgKind {
        match self {
            Self::Entity(_) => rules::ArgKind::Entity,
            Self::Entities(_) => rules::ArgKind::Entities,
            Self::Player(_) => rules::ArgKind::Player,
            Self::Amount(_) => rules::ArgKind::Amount,
            Self::Event(_) => rules::ArgKind::Event,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;

    use super::ArgValue;

    #[test]
    fn test_usage() {
        let entity = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = rules::EventId("601.2a/cast");
        assert_eq!(ArgValue::Entity(entity).entity(), Some(entity));
        assert_eq!(ArgValue::Entity(entity).kind(), rules::ArgKind::Entity);
        assert_eq!(
            ArgValue::Entities(vec![entity]).entities(),
            Some(vec![entity])
        );
        assert_eq!(
            ArgValue::Entities(vec![entity]).kind(),
            rules::ArgKind::Entities
        );
        assert_eq!(ArgValue::Player(player).player(), Some(player));
        assert_eq!(ArgValue::Player(player).kind(), rules::ArgKind::Player);
        assert_eq!(ArgValue::Amount(3).amount(), Some(3));
        assert_eq!(ArgValue::Amount(3).kind(), rules::ArgKind::Amount);
        assert_eq!(ArgValue::Event(event).event(), Some(event));
        assert_eq!(ArgValue::Event(event).kind(), rules::ArgKind::Event);
        assert_eq!(ArgValue::Entity(entity).player(), None);
        assert_eq!(ArgValue::Amount(3).entity(), None);
    }
}
