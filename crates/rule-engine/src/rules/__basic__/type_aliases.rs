use bevy::prelude::{Entity, EntityRef};

pub type TargetOf = for<'a> fn(&'a EntityRef<'a>) -> Option<Entity>;
