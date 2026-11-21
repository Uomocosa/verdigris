use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const JUMP: testing::Card = testing::Card {
    name: testing::TestCardName("Jump"),
    definition: || {
        cards::instant("Jump", cost![U])
            .spell(effects::gains(cards::flying()).to(effects::target(1, CREATURE)))
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::JUMP;

    #[test]
    fn test_usage() {
        let jump = (JUMP.definition)();
        assert_eq!(*jump.name, "Jump");
        assert_eq!(jump.types, vec![cards::CardType::Instant]);
        assert_eq!(jump.cost, cost![U]);
        assert_eq!(jump.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            jump.effect,
            Some(effects::gains(cards::flying()).to(effects::target(1, CREATURE)))
        );
    }
}
