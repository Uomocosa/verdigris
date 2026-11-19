#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{
    Damage, DealtByDeathtouch, InZone, Life, Lost, Owner, Player, Zone, ZoneContents,
};
pub use basic::messages::{DamageDealt, Destruction};
