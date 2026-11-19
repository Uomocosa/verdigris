use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn giant_growth() -> cards::CardDefinition {
    cards::instant("Giant Growth", cost![G])
        .spell(effects::gets(3, 3).to(effects::target(1, CREATURE)))
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::giant_growth;

    #[test]
    fn test_usage() {
        let growth = giant_growth();
        assert_eq!(*growth.name, "Giant Growth");
        assert_eq!(growth.types, vec![cards::CardType::Instant]);
        assert_eq!(growth.cost, cost![G]);
        assert_eq!(growth.colors(), vec![cards::Color::Green]);
        assert_eq!(
            growth.effect,
            Some(effects::gets(3, 3).to(effects::target(1, CREATURE)))
        );
    }
}
