use crate::abilities;
use crate::cards;
use crate::effects;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub event: abilities::TriggerEvent,
    pub intervening_if: Option<rules::Condition>,
    pub effect: effects::Effect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: abilities::ActivationCost,
    pub produces: cards::ManaType,
}
