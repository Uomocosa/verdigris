use derive_more::Deref;

use crate::abilities;
use crate::cards;
use crate::events;
use crate::keywords;
use crate::players;
use crate::rules;
use crate::statics;
use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref)]
pub struct Filter(rules::Condition);

#[rustfmt::skip]
impl From<rules::Players> for Filter {
    fn from(players: rules::Players) -> Self { Self::player() & Self::controlled_by(players) }
}

#[rustfmt::skip]
impl From<rules::You> for Filter {
    fn from(you: rules::You) -> Self { Self::from(rules::Players::from(you)) }
}

#[rustfmt::skip]
impl From<&statics::All> for Filter {
    fn from(all: &statics::All) -> Self { (**all).clone() }
}

#[rustfmt::skip]
impl From<&keywords::EnchantFilter> for Filter {
    fn from(enchant: &keywords::EnchantFilter) -> Self { (**enchant).clone() }
}

impl Filter {
    #[must_use]
    pub const fn card_type(card: cards::CardType) -> Self {
        Self(rules::card_type(card))
    }

    #[must_use]
    pub const fn color(color: cards::Color) -> Self {
        Self(rules::color(color))
    }

    #[must_use]
    pub const fn subtype(subtype: cards::SubtypeId) -> Self {
        Self(rules::subtype(subtype))
    }

    #[must_use]
    pub const fn supertype(supertype: cards::Supertype) -> Self {
        Self(rules::supertype(supertype))
    }

    #[must_use]
    pub fn permanent() -> Self {
        Self(rules::has::<cards::OnBattlefield>())
    }

    #[must_use]
    pub const fn spell() -> Self {
        Self(rules::spell())
    }

    #[must_use]
    pub fn player() -> Self {
        Self(rules::has::<players::Player>())
    }

    #[must_use]
    pub fn source() -> Self {
        Self(rules::is(rules::ArgRef::SourceOf(
            events::AbilityContext::SPELL_OR_ABILITY,
        )))
    }

    #[must_use]
    pub const fn enchanted() -> Self {
        Self(rules::enchanted())
    }

    #[must_use]
    pub fn controlled_by(players: impl Into<rules::Players>) -> Self {
        Self(rules::controlled_by(players))
    }

    #[must_use]
    pub fn from_in_zone(zone: zones::Zone, role: impl Into<rules::ArgRef>) -> Self {
        Self(rules::in_zone(zone, role))
    }

    #[must_use]
    pub fn token() -> Self {
        Self(rules::has::<cards::Token>())
    }

    #[must_use]
    pub fn attacking() -> Self {
        Self(rules::has::<cards::Attacking>())
    }

    #[must_use]
    pub const fn has_keyword(id: keywords::KeywordId) -> Self {
        Self(rules::has_keyword(id))
    }
}

#[rustfmt::skip]
impl std::ops::Not for Filter {
    type Output = Self;
    fn not(self) -> Self { Self(!(*self).clone()) }
}

#[rustfmt::skip]
impl std::ops::BitAnd for Filter {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { Self((*self).clone() & (*rhs).clone()) }
}

#[rustfmt::skip]
impl std::ops::BitOr for Filter {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self((*self).clone() | (*rhs).clone()) }
}

#[rustfmt::skip]
impl Filter {
    #[must_use]
    pub fn of(self, who: impl Into<rules::Subject>) -> rules::Condition {
        (*self).clone().of(who)
    }

    #[must_use]
    pub fn and(self, other: Self) -> Self { self & other }

    #[must_use]
    pub fn or(self, other: Self) -> Self { self | other }

    #[must_use]
    pub const fn enters(self) -> abilities::TriggerEvent { abilities::TriggerEvent::Enters(self) }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::rules;

    #[test]
    fn test_usage() {
        let filter =
            effects::Filter::permanent() & effects::Filter::card_type(cards::CardType::Creature);
        assert_eq!(
            filter,
            effects::Filter::permanent() & effects::Filter::card_type(cards::CardType::Creature)
        );
        assert_eq!(
            effects::Filter::from(rules::Players::You),
            effects::Filter::player() & effects::Filter::controlled_by(rules::Players::You)
        );
        assert_eq!(!effects::Filter::token(), !effects::Filter::token());
    }
}
