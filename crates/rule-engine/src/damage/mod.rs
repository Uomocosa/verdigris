#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::DealtByDeathtouch;
pub use basic::messages::{DamageDealt, Destruction};

mod damage;
pub use damage::Damage;
