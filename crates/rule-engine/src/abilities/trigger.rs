use crate::abilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trigger {
    pub event: abilities::TriggerEvent,
    pub intervening_if: Option<abilities::Condition>,
}

impl From<abilities::TriggerEvent> for Trigger {
    fn from(event: abilities::TriggerEvent) -> Self {
        Self {
            event,
            intervening_if: None,
        }
    }
}
