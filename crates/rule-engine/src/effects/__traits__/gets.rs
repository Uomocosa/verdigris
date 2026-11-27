use crate::effects;

pub trait Gets {
    #[must_use]
    fn kind(self) -> effects::EffectKind;
}

#[cfg(test)]
mod tests {
    use crate::counters;
    use crate::effects;
    use crate::effects::traits::gets::Gets;

    #[test]
    fn test_usage() {
        assert_eq!(
            (3 * counters::POISON).kind(),
            effects::EffectKind::Poison(3 * counters::POISON)
        );
    }
}
