#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::DealtByDeathtouch;
pub use basic::structs::AssignedDamage;

mod damage;
pub use damage::Damage;
