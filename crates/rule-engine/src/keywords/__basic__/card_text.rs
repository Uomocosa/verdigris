use crate::effects;
use crate::keywords;

#[must_use]
pub fn enchant(filter: effects::Filter) -> keywords::Keyword {
    keywords::Enchant::keyword(filter)
}

#[must_use]
pub fn indestructible() -> keywords::Keyword {
    keywords::Indestructible::keyword()
}

#[must_use]
pub fn deathtouch() -> keywords::Keyword {
    keywords::Deathtouch::keyword()
}

#[must_use]
pub fn lifelink() -> keywords::Keyword {
    keywords::Lifelink::keyword()
}

#[must_use]
pub fn trample() -> keywords::Keyword {
    keywords::Trample::keyword()
}

#[must_use]
pub fn flying() -> keywords::Keyword {
    keywords::Flying::keyword()
}

#[must_use]
pub fn reach() -> keywords::Keyword {
    keywords::Reach::keyword()
}

#[must_use]
pub fn vigilance() -> keywords::Keyword {
    keywords::Vigilance::keyword()
}

#[must_use]
pub fn defender() -> keywords::Keyword {
    keywords::Defender::keyword()
}

#[must_use]
pub fn haste() -> keywords::Keyword {
    keywords::Haste::keyword()
}

#[must_use]
pub fn hexproof() -> keywords::Keyword {
    keywords::Hexproof::keyword()
}

#[must_use]
pub fn first_strike() -> keywords::Keyword {
    keywords::FirstStrike::keyword()
}

#[must_use]
pub fn double_strike() -> keywords::Keyword {
    keywords::DoubleStrike::keyword()
}

#[must_use]
pub fn infect() -> keywords::Keyword {
    keywords::Infect::keyword()
}
