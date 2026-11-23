use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const BORDERLAND_MARAUDER: testing::Card = testing::Card {
    name: testing::TestCardName("Borderland Marauder"),
    definition: || {
        card! {
            cards::creature("Borderland Marauder", cost![1, R], pt!(1, 2));
            abilities::whenever(abilities::TriggerEvent::attacks(), effects::gets(2, 0).to(THIS)),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::THIS;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::BORDERLAND_MARAUDER;

    #[test]
    fn test_usage() {
        let marauder = (BORDERLAND_MARAUDER.definition)();
        assert_eq!(*marauder.name, "Borderland Marauder");
        assert_eq!(marauder.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(marauder.cost, cost![1, R]);
        assert_eq!(marauder.power_toughness, Some(pt!(1, 2)));
        assert_eq!(
            marauder.triggered_abilities().copied().collect::<Vec<_>>(),
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::attacks(),
                intervening_if: None,
                effect: effects::gets(2, 0).to(THIS),
            }]
        );
    }
}
