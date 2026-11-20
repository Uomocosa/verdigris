use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleSet {
    Turn,
    PriorityAction,
    Resolution,
    StateBased,
    StepEnd,
    StepStarted,
    Cleanup,
    TurnEnd,
}
