pub trait Dealt {
    type Builder;

    #[must_use]
    fn build(self) -> Self::Builder;
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::rules;
    use crate::rules::traits::dealt::Dealt;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::COMBAT_DAMAGE.build(),
            abilities::DealsDamage::from(true)
        );
    }
}
