use crate::effects;
use crate::statics;

#[must_use]
pub const fn all(filter: effects::Filter) -> statics::All {
    statics::All(filter)
}
