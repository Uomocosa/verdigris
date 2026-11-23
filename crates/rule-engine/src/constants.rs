use crate::cards;
use crate::effects;
use crate::rules;
use crate::turns;
use crate::zones;

pub const YOU: effects::PlayerRef = effects::PlayerRef::You;
pub const OPPONENT: effects::PlayerRef = effects::PlayerRef::Opponent;
pub const ANY: effects::PlayerRef = effects::PlayerRef::Any;

pub const ANOTHER_CREATURE: cards::ObjectFilter = cards::ObjectFilter::AnotherCreature;

/// 514.1 Maximum hand size (normally seven).
pub const MAX_HAND_SIZE: usize = 7;

/// Guards the 117.5 loop against an endless chain of state-based actions.
pub const MAX_STATE_BASED_PASSES: u32 = 64;

pub const HAND: zones::Zone = zones::Zone::Hand;
pub const BATTLEFIELD: zones::Zone = zones::Zone::Battlefield;
pub const GRAVEYARD: zones::Zone = zones::Zone::Graveyard;
pub const LIBRARY: zones::Zone = zones::Zone::Library;
pub const EXILE: zones::Zone = zones::Zone::Exile;
pub const STACK: zones::SharedZone = zones::SharedZone::Stack;

pub const UNTAP: turns::Step = turns::Step::Untap;
pub const UPKEEP: turns::Step = turns::Step::Upkeep;
pub const DRAW: turns::Step = turns::Step::Draw;
pub const MAIN_1: turns::Step = turns::Step::PrecombatMain;
pub const BEGINNING_OF_COMBAT: turns::Step = turns::Step::BeginningOfCombat;
pub const DECLARE_ATTACKERS: turns::Step = turns::Step::DeclareAttackers;
pub const DECLARE_BLOCKERS: turns::Step = turns::Step::DeclareBlockers;
pub const COMBAT_DAMAGE: turns::Step = turns::Step::CombatDamage;
pub const END_OF_COMBAT: turns::Step = turns::Step::EndOfCombat;
pub const MAIN_2: turns::Step = turns::Step::PostcombatMain;
pub const END: turns::Step = turns::Step::End;
pub const CLEANUP: turns::Step = turns::Step::Cleanup;

/// 508.1f The active player taps the chosen creatures.
pub const TAP_ATTACKERS: rules::ActionId = rules::ActionId("508.1f/tap");

/// 502.3 The active player untaps permanents simultaneously.
pub const UNTAP_STEP: rules::ActionId = rules::ActionId("502.3/untap");

pub const CREATURE: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Creature,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};
pub const ENCHANTMENT: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Enchantment,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};
pub const ANY_TARGET: effects::Filter = effects::Filter {
    kind: effects::FilterKind::AnyTarget,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};
pub const PLAYER: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Player,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
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
    attacking: false,
    nontoken: false,
    object: None,
};
pub const CREATURE_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::CreatureSpell,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};
pub const NONCREATURE_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::NoncreatureSpell,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};
pub const INSTANT_SPELL: effects::Filter = effects::Filter {
    kind: effects::FilterKind::InstantSpell,
    controlled_by: None,
    excluded_color: None,
    attacking: false,
    nontoken: false,
    object: None,
};

/// 105.1 There are five colors in the Magic game: white, blue, black, red, and green.
/// 202.2 An object is the color or colors of the mana symbols in its mana cost.
pub const NONBLACK_CREATURE: effects::Filter = effects::Filter {
    kind: effects::FilterKind::Creature,
    controlled_by: None,
    excluded_color: Some(cards::Color::Black),
    attacking: false,
    nontoken: false,
    object: None,
};

/// The source of “it” in printed text such as “it gets +2/+0”: the ability's own source.
pub const THIS: effects::Target = effects::Target {
    count: 1,
    filter: effects::Filter {
        kind: effects::FilterKind::Creature,
        controlled_by: None,
        excluded_color: None,
        attacking: false,
        nontoken: false,
        object: Some(effects::ObjectRelation::ThisObject),
    },
};
