use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn murder() -> cards::CardDefinition {
    cards::instant("Murder", cost![1, B, B])
        .spell(effects::destroy().to(effects::target(1, CREATURE)))
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::effects;

    use super::murder;

    #[test]
    fn test_usage() {
        let removal = murder();
        assert_eq!(*removal.name, "Murder");
        assert_eq!(removal.types, vec![cards::CardType::Instant]);
        assert_eq!(
            removal.effect,
            Some(effects::destroy().to(effects::target(1, CREATURE)))
        );
    }
}
