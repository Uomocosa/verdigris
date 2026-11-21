use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trigger {
    pub event: cards::TriggerEvent,
    pub intervening_if: Option<cards::Condition>,
}

impl From<cards::TriggerEvent> for Trigger {
    fn from(event: cards::TriggerEvent) -> Self {
        Self {
            event,
            intervening_if: None,
        }
    }
}
