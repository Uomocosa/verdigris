use crate::ANY_TARGET;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const LIGHTNING_BOLT: testing::Card = testing::Card {
    name: "Lightning Bolt",
    definition: || {
        cards::instant("Lightning Bolt", cost![R])
            .spell(effects::damage(3).to(effects::target(1, ANY_TARGET)))
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;

    use super::LIGHTNING_BOLT;

    #[test]
    fn test_usage() {
        let bolt = (LIGHTNING_BOLT.definition)();
        assert_eq!(*bolt.name, "Lightning Bolt");
        assert_eq!(bolt.types, vec![cards::CardType::Instant]);
        assert_eq!(
            bolt.effect,
            Some(effects::damage(3).to(effects::target(1, crate::ANY_TARGET)))
        );
    }
}
