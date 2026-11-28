use crate::abilities;
use crate::creature;
use crate::effects;
use crate::rules;
use crate::statics;

#[must_use]
pub fn all(filter: impl Into<effects::Filter>) -> statics::All {
    statics::All(filter.into())
}

#[must_use]
pub fn you() -> statics::All {
    statics::all(effects::Filter::player() & effects::Filter::controlled_by(rules::Players::You))
}

#[must_use]
pub fn each_opponent() -> statics::All {
    statics::all(
        effects::Filter::player() & effects::Filter::controlled_by(rules::Players::Opponents),
    )
}

#[must_use]
pub fn this() -> statics::All {
    statics::all(creature() & effects::Filter::source())
}

#[must_use]
pub fn enchanted() -> statics::All {
    statics::all(creature() & effects::Filter::enchanted())
}

#[must_use]
pub const fn enters_tapped() -> abilities::Replacement {
    abilities::Replacement::EntersTapped
}

#[must_use]
pub fn if_a(filter: impl Into<effects::Filter>) -> statics::If {
    statics::If(all(filter.into()))
}

#[must_use]
pub fn if_you() -> statics::If {
    statics::If(you())
}

#[must_use]
pub fn if_a_player() -> statics::If {
    statics::If(all(effects::Filter::player()))
}

#[must_use]
pub fn if_an_opponent() -> statics::If {
    statics::If(each_opponent())
}
