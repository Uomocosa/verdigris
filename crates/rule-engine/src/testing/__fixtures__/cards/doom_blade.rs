use crate::NONBLACK_CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DOOM_BLADE: testing::Card = testing::Card {
    name: testing::TestCardName("Doom Blade"),
    definition: || {
        cards::instant("Doom Blade", cost![1, B])
            .spell(effects::destroy().to(effects::target(1, NONBLACK_CREATURE)))
    },
};

#[cfg(test)]
mod tests {
    use crate::NONBLACK_CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::DOOM_BLADE;

    #[test]
    fn test_usage() {
        let removal = (DOOM_BLADE.definition)();
        assert_eq!(*removal.name, "Doom Blade");
        assert_eq!(removal.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(removal.cost, cost![1, B]);
        assert_eq!(removal.colors(), vec![cards::Color::Black]);
        assert_eq!(
            removal.effect,
            Some(effects::destroy().to(effects::target(1, NONBLACK_CREATURE)))
        );
    }
}
