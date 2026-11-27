use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceWhy {
    Duplicate,
    NotAllowed(Entity),
    TooFew { min: u32 },
    TooMany { max: u32 },
    MustOrderAll,
}
