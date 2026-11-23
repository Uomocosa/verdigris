#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Characteristic, Combine, Comparison, Question, Role};
pub use basic::newtypes::{ClauseKey, ComponentKey, RuleId, StepId};
pub use basic::predicate_builders::{at_least, at_most, equal_to, has, other};
pub use basic::structs::Clause;

#[path = "__traits__/mod.rs"]
pub mod traits;

mod answer;
pub use answer::Answer;

mod attack_ctx;
pub use attack_ctx::AttackCtx;

mod block_ctx;
pub use block_ctx::BlockCtx;

mod clause_id;
pub use clause_id::ClauseId;

mod predicate;
pub use predicate::Predicate;

mod predicate_registry;
pub use predicate_registry::PredicateRegistry;

mod rules;
pub use rules::Rules;

mod ignores_summoning_sickness;
pub use ignores_summoning_sickness::IgnoresSummoningSickness;
mod must_attack;
pub use must_attack::MustAttack;
mod must_block;
pub use must_block::MustBlock;
mod taps_when_attacking;
pub use taps_when_attacking::TapsWhenAttacking;
