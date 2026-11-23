use crate::abilities;
use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub event: abilities::TriggerEvent,
    pub intervening_if: Option<abilities::Condition>,
    pub effect: effects::Effect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continuous {
    pub filter: effects::Filter,
    pub modification: effects::Modification,
}
