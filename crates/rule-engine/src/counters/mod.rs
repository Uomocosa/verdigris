#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::counter_amount::CounterAmount;
pub use basic::counter_id::CounterId;
pub use basic::minus_one_minus_one::MINUS_ONE_MINUS_ONE;
pub use basic::poison::POISON;

mod counters;
pub use counters::Counters;
