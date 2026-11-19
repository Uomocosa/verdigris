use crate::cards;
use crate::effects;
use crate::zones;

pub const YOU: effects::PlayerRef = effects::PlayerRef::You;
pub const OPPONENT: effects::PlayerRef = effects::PlayerRef::Opponent;
pub const ANY: effects::PlayerRef = effects::PlayerRef::Any;

pub const HAND: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Hand,
    of: effects::PlayerRef::Any,
};
pub const BATTLEFIELD: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Battlefield,
    of: effects::PlayerRef::Any,
};
pub const GRAVEYARD: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Graveyard,
    of: effects::PlayerRef::Any,
};
pub const LIBRARY: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Library,
    of: effects::PlayerRef::Any,
};
pub const EXILE: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Exile,
    of: effects::PlayerRef::Any,
};
pub const STACK: effects::ZoneRef = effects::ZoneRef {
    zone: zones::Zone::Stack,
    of: effects::PlayerRef::Any,
};

pub const CREATURE: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Creature,
    controlled_by: None,
    excluded_color: None,
};
pub const ANY_TARGET: effects::Filter = effects::Filter {
    kind: effects::FilterKind::AnyTarget,
    controlled_by: None,
    excluded_color: None,
};
pub const PLAYER: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Player,
    controlled_by: None,
    excluded_color: None,
};

/// 115.5 A spell or ability on the stack is an illegal target for itself.
///
/// 115.2 Only permanents are legal targets for spells and abilities, unless a spell or ability
/// specifies that it can target an object in another zone or a player, or targets an object that
/// can't exist on the battlefield, such as a spell.
pub const SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Spell,
    controlled_by: None,
    excluded_color: None,
};
pub const CREATURE_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::CreatureSpell,
    controlled_by: None,
    excluded_color: None,
};
pub const NONCREATURE_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::NoncreatureSpell,
    controlled_by: None,
    excluded_color: None,
};
pub const INSTANT_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::InstantSpell,
    controlled_by: None,
    excluded_color: None,
};

/// 105.1 There are five colors in the Magic game: white, blue, black, red, and green.
/// 202.2 An object is the color or colors of the mana symbols in its mana cost.
pub const NONBLACK_CREATURE: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Creature,
    controlled_by: None,
    excluded_color: Some(cards::Color::Black),
};
