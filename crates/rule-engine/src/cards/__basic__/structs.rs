use crate::cards;
use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PowerToughness {
    pub power: i32,
    pub toughness: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub event: cards::TriggerEvent,
    pub intervening_if: Option<cards::Condition>,
    pub effect: effects::Effect,
}
