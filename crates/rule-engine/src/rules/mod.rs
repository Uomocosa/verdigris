#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Answer, Combine, Question};

#[path = "__traits__/mod.rs"]
pub mod traits;

mod rules;
pub use rules::Rules;

mod taps_when_attacking;
pub use taps_when_attacking::TapsWhenAttacking;
