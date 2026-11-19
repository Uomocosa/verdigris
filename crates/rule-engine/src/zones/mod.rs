#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{InZone, ZoneContents};

mod zone;
pub use zone::Zone;
