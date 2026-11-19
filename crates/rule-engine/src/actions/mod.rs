#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Action, Choice};
pub use basic::messages::{PlayerAction, Rejected};
pub use basic::resources::PendingChoice;
pub use basic::structs::{DiscardChoice, TargetChoice};
