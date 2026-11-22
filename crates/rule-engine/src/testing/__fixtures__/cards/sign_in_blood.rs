use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const SIGN_IN_BLOOD: testing::Card = testing::Card {
    name: testing::TestCardName("Sign in Blood"),
    definition: || {
        cards::sorcery("Sign in Blood", cost![B, B])
            .spell(effects::draw(2).and(effects::lose_life(2)).to(effects::target(1, PLAYER)))
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::SIGN_IN_BLOOD;

    #[test]
    fn test_usage() {
        let spell = (SIGN_IN_BLOOD.definition)();
        assert_eq!(*spell.name, "Sign in Blood");
        assert_eq!(spell.type_line.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![B, B]);
        assert_eq!(spell.colors(), vec![cards::Color::Black]);
        assert_eq!(
            spell.spell_ability().copied(),
            Some(effects::draw(2).and(effects::lose_life(2)).to(effects::target(1, PLAYER)))
        );
    }
}
