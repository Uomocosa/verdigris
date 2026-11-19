use crate::ANY_TARGET;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn lightning_bolt() -> cards::CardDefinition {
    cards::instant("Lightning Bolt", cost![R])
        .spell(effects::damage(3).to(effects::target(1, ANY_TARGET)))
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;

    use super::lightning_bolt;

    #[test]
    fn test_usage() {
        let bolt = lightning_bolt();
        assert_eq!(*bolt.name, "Lightning Bolt");
        assert_eq!(bolt.types, vec![cards::CardType::Instant]);
        assert_eq!(
            bolt.effect,
            Some(effects::damage(3).to(effects::target(1, crate::ANY_TARGET)))
        );
    }
}
