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
