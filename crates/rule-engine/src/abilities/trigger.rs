use crate::abilities;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    pub event: abilities::TriggerEvent,
    pub intervening_if: Option<rules::Condition>,
}

impl From<abilities::TriggerEvent> for Trigger {
    fn from(event: abilities::TriggerEvent) -> Self {
        Self {
            event,
            intervening_if: None,
        }
    }
}
