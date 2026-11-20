use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleSet {
    Turn,
    /// Every action a player takes while holding priority (casting, declaring
    /// attackers, and soon land plays and mana abilities).
    ///
    /// 117.1. Unless a spell or ability is instructing a player to take an action, which player can take actions at any given time is determined by a system of priority. The player with priority may cast spells, activate abilities, and take special actions.
    PriorityAction,
    Resolution,
    StateBased,
    StepEnd,
    StepStarted,
    Cleanup,
    TurnEnd,
}
