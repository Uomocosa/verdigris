use crate::abilities;
use crate::effects;
use crate::statics;

#[must_use]
pub const fn all(filter: effects::Filter) -> statics::All {
    statics::All(filter)
}

#[must_use]
pub const fn this() -> statics::All {
    statics::All(effects::Filter {
        kind: effects::FilterKind::Creature,
        controlled_by: None,
        excluded_color: None,
        attacking: false,
        nontoken: false,
        is_this_object: true,
    })
}

#[must_use]
pub const fn enters_tapped() -> abilities::Replacement {
    abilities::Replacement::EntersTapped
}
