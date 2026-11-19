use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn sign_in_blood() -> cards::CardDefinition {
    cards::sorcery("Sign in Blood", cost![B, B]).spell(
        effects::draw(2)
            .and(effects::lose_life(2))
            .to(effects::target(1, PLAYER)),
    )
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::sign_in_blood;

    #[test]
    fn test_usage() {
        let spell = sign_in_blood();
        assert_eq!(*spell.name, "Sign in Blood");
        assert_eq!(spell.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![B, B]);
        assert_eq!(spell.colors(), vec![cards::Color::Black]);
        assert_eq!(
            spell.effect,
            Some(
                effects::draw(2)
                    .and(effects::lose_life(2))
                    .to(effects::target(1, PLAYER))
            )
        );
    }
}
