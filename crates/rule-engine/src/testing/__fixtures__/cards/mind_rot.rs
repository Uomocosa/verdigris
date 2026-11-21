use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const MIND_ROT: testing::Card = testing::Card {
    name: testing::TestCardName("Mind Rot"),
    definition: || {
        cards::sorcery("Mind Rot", cost![2, B])
            .spell(effects::discard(2).to(effects::target(1, PLAYER)))
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::MIND_ROT;

    #[test]
    fn test_usage() {
        let spell = (MIND_ROT.definition)();
        assert_eq!(*spell.name, "Mind Rot");
        assert_eq!(spell.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![2, B]);
        assert_eq!(spell.colors(), vec![cards::Color::Black]);
        assert_eq!(
            spell.effect,
            Some(effects::discard(2).to(effects::target(1, PLAYER)))
        );
    }
}
