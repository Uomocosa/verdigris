use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

#[must_use]
pub fn all(filter: impl Into<effects::Filter>) -> statics::All {
    statics::All(filter.into())
}

#[must_use]
pub fn you() -> statics::All {
    statics::all(effects::Filter::player(rules::Players::You))
}

#[must_use]
pub fn each_opponent() -> statics::All {
    statics::all(effects::Filter::player(rules::Players::Opponents))
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
