use crate::cards;
use crate::rules;
use crate::turns;
use crate::zones;

pub const YOU: rules::You = rules::You;
pub const OPPONENT: rules::Players = rules::Players::Opponents;
pub const PLAYER: rules::Players = rules::Players::All;

pub const ANOTHER_CREATURE: cards::ObjectFilter = cards::ObjectFilter::AnotherCreature;

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

/// The source of “it” in printed text such as “it gets +2/+0”: the ability's own source.
pub const THIS: rules::This = rules::This;

pub const LOSES_LIFE: rules::ResultId = rules::ResultId("120.3a/loses-life");

pub const FIRST_COMBAT_DAMAGE_STEP: rules::DefinitionId = rules::DefinitionId("510.4/first-step");
pub const ASSIGNS_AGAIN: rules::DefinitionId = rules::DefinitionId("510.4/assigns-again");
