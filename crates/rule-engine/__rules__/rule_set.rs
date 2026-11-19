use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleSet {
    Turn,
    Casting,
    Resolution,
    StateBased,
    Cleanup,
}
