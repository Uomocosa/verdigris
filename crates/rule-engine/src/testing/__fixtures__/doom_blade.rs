use crate::NONBLACK_CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn doom_blade() -> cards::CardDefinition {
    cards::instant("Doom Blade", cost![1, B])
        .spell(effects::destroy().to(effects::target(1, NONBLACK_CREATURE)))
}

#[cfg(test)]
mod tests {
    use crate::NONBLACK_CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::doom_blade;

    #[test]
    fn test_usage() {
        let removal = doom_blade();
        assert_eq!(*removal.name, "Doom Blade");
        assert_eq!(removal.types, vec![cards::CardType::Instant]);
        assert_eq!(removal.cost, cost![1, B]);
        assert_eq!(removal.colors(), vec![cards::Color::Black]);
        assert_eq!(
            removal.effect,
            Some(effects::destroy().to(effects::target(1, NONBLACK_CREATURE)))
        );
    }
}
