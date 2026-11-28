use crate::cards;
use crate::effects;
use crate::rules;

/// A creature is a permanent of the creature card type on the battlefield.
///
/// 109.2. If a spell or ability uses a description of an object that includes a card type
/// or subtype, but doesn’t include the word “card,” “spell,” “source,” or “scheme,” it
/// means a permanent of that card type or subtype on the battlefield.
#[must_use]
pub fn creature() -> effects::Filter {
    effects::Filter::permanent() & effects::Filter::card_type(cards::CardType::Creature)
}

#[must_use]
pub fn enchantment() -> effects::Filter {
    effects::Filter::permanent() & effects::Filter::card_type(cards::CardType::Enchantment)
}

/// 105.1 There are five colors in the Magic game: white, blue, black, red, and green.
/// 202.2 An object is the color or colors of the mana symbols in its mana cost.
#[must_use]
pub fn nonblack_creature() -> effects::Filter {
    creature() & !effects::Filter::color(cards::Color::Black)
}

#[must_use]
pub fn any_target() -> effects::Filter {
    creature() | effects::Filter::player()
}

#[must_use]
pub fn another_creature() -> effects::Filter {
    creature() & !effects::Filter::source()
}

/// 115.5 A spell or ability on the stack is an illegal target for itself.
///
/// 115.2 Only permanents are legal targets for spells and abilities, unless a spell or ability
/// specifies that it can target an object in another zone or a player, or targets an object that
/// can't exist on the battlefield, such as a spell.
///
/// 112.1 A spell is a card on the stack.
#[must_use]
pub const fn spell() -> effects::Filter {
    effects::Filter::spell()
}

#[must_use]
pub fn creature_spell() -> effects::Filter {
    effects::Filter::spell() & effects::Filter::card_type(cards::CardType::Creature)
}

#[must_use]
pub fn noncreature_spell() -> effects::Filter {
    effects::Filter::spell() & !effects::Filter::card_type(cards::CardType::Creature)
}

#[must_use]
pub fn instant_spell() -> effects::Filter {
    effects::Filter::spell() & effects::Filter::card_type(cards::CardType::Instant)
}

#[must_use]
pub fn controlled_by(players: impl Into<rules::Players>) -> effects::Filter {
    effects::Filter::controlled_by(players)
}

#[must_use]
pub fn owned_by(players: impl Into<rules::Players>) -> effects::Filter {
    effects::Filter::owned_by(players)
}

#[must_use]
pub fn token() -> effects::Filter {
    effects::Filter::token()
}

#[must_use]
pub fn card() -> effects::Filter {
    effects::Filter::card()
}

#[must_use]
pub fn attacking() -> effects::Filter {
    effects::Filter::attacking()
}

#[must_use]
pub fn player() -> effects::Filter {
    effects::Filter::player()
}
