use crate::abilities;
use crate::effects;
use crate::statics;

#[must_use]
pub const fn all(filter: effects::Filter) -> statics::All {
    statics::All(filter)
}

#[must_use]
pub const fn you() -> statics::All {
    statics::all(effects::Filter::player(effects::PlayerRef::You))
}

#[must_use]
pub const fn each_opponent() -> statics::All {
    statics::all(effects::Filter::player(effects::PlayerRef::Opponent))
}

#[must_use]
pub const fn this() -> statics::All {
    statics::All(effects::Filter {
        kind: effects::FilterKind::Creature,
        controlled_by: None,
        excluded_color: None,
        attacking: false,
        nontoken: false,
        object: Some(effects::ObjectRelation::ThisObject),
    })
}

#[must_use]
pub const fn enchanted() -> statics::All {
    statics::All(effects::Filter {
        kind: effects::FilterKind::Creature,
        controlled_by: None,
        excluded_color: None,
        attacking: false,
        nontoken: false,
        object: Some(effects::ObjectRelation::EnchantedObject),
    })
}

#[must_use]
pub const fn enters_tapped() -> abilities::Replacement {
    abilities::Replacement::EntersTapped
}
