use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFilter {
    AnotherCreature,
}

#[rustfmt::skip]
impl ObjectFilter {
    #[must_use]
    pub const fn enters(self) -> cards::TriggerEvent { cards::TriggerEvent::Enters(self) }
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::ObjectFilter;

    #[test]
    fn test_usage() {
        assert_eq!(
            ObjectFilter::AnotherCreature.enters(),
            cards::TriggerEvent::Enters(ObjectFilter::AnotherCreature)
        );
    }
}
