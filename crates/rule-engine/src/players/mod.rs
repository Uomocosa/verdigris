#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{AttemptedDrawFromEmptyLibrary, Life, Lost, Player};
pub use basic::messages::{DiscardRequested, DrawRequested, LifeLost};
