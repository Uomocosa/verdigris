use crate::abilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFilter {
    AnotherCreature,
}

#[rustfmt::skip]
impl ObjectFilter {
    #[must_use]
    pub const fn enters(self) -> abilities::TriggerEvent { abilities::TriggerEvent::Enters(self) }
}

#[cfg(test)]
mod tests {
    use crate::abilities;

    use super::ObjectFilter;

    #[test]
    fn test_usage() {
        assert_eq!(
            ObjectFilter::AnotherCreature.enters(),
            abilities::TriggerEvent::Enters(ObjectFilter::AnotherCreature)
        );
    }
}
