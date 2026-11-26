use crate::effects;
use crate::keywords;

#[must_use]
pub fn enchant(filter: effects::Filter) -> keywords::Keyword {
    keywords::enchant::r702_5a_enchant(filter)
}

#[must_use]
pub fn indestructible() -> keywords::Keyword {
    keywords::indestructible::r702_12b_indestructible()
}

#[must_use]
pub fn deathtouch() -> keywords::Keyword {
    keywords::deathtouch::r702_2c_deathtouch()
}

#[must_use]
pub fn lifelink() -> keywords::Keyword {
    keywords::lifelink::r702_15b_lifelink()
}

#[must_use]
pub fn trample() -> keywords::Keyword {
    keywords::trample::r702_19b_trample()
}

#[must_use]
pub fn flying() -> keywords::Keyword {
    keywords::flying::r702_9b_flying()
}

#[must_use]
pub fn reach() -> keywords::Keyword {
    keywords::reach::r702_17b_reach()
}

#[must_use]
pub fn vigilance() -> keywords::Keyword {
    keywords::vigilance::r702_20a_vigilance()
}

#[must_use]
pub fn defender() -> keywords::Keyword {
    keywords::defender::r702_3a_defender()
}

#[must_use]
pub fn haste() -> keywords::Keyword {
    keywords::haste::r702_10a_haste()
}

#[must_use]
pub fn hexproof() -> keywords::Keyword {
    keywords::hexproof::r702_11b_hexproof()
}

#[must_use]
pub fn first_strike() -> keywords::Keyword {
    keywords::first_strike::r702_7b_first_strike()
}

#[must_use]
pub fn double_strike() -> keywords::Keyword {
    keywords::double_strike::r702_4b_double_strike()
}

#[must_use]
pub fn infect() -> keywords::Keyword {
    keywords::infect::r702_90b_infect()
}
