#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Combine, Question};

#[path = "__traits__/mod.rs"]
pub mod traits;

mod answer;
pub use answer::Answer;

mod rules;
pub use rules::Rules;

mod ignores_summoning_sickness;
pub use ignores_summoning_sickness::IgnoresSummoningSickness;
mod taps_when_attacking;
pub use taps_when_attacking::TapsWhenAttacking;
