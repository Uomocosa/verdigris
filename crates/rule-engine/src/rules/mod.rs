#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Characteristic, Comparison, Role};
pub use basic::newtypes::{ClauseKey, ComponentKey, RuleId, StepId};
pub use basic::predicate_builders::{at_least, at_most, equal_to, has, other};
pub use basic::structs::Clause;

#[path = "__traits__/mod.rs"]
pub mod traits;

mod attack_ctx;
pub use attack_ctx::AttackCtx;

mod block_ctx;
pub use block_ctx::BlockCtx;

mod clause_id;
pub use clause_id::ClauseId;

pub mod declare_blocker;
pub use declare_blocker::DeclareBlocker;

pub mod declare_attacker;
pub use declare_attacker::DeclareAttacker;

mod predicate;
pub use predicate::Predicate;

mod predicate_registry;
pub use predicate_registry::PredicateRegistry;

mod rules;
pub use rules::Rules;
