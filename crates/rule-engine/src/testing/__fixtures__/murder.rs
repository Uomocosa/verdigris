use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const MURDER: testing::Card = testing::Card {
    name: testing::TestCardName("Murder"),
    definition: || {
        cards::instant("Murder", cost![1, B, B])
            .spell(effects::destroy().to(effects::target(1, CREATURE)))
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::effects;

    use super::MURDER;

    #[test]
    fn test_usage() {
        let removal = (MURDER.definition)();
        assert_eq!(*removal.name, "Murder");
        assert_eq!(removal.types, vec![cards::CardType::Instant]);
        assert_eq!(
            removal.effect,
            Some(effects::destroy().to(effects::target(1, CREATURE)))
        );
    }
}
