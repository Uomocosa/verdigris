use crate::UPKEEP;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::testing;
use crate::turns;

pub const FELIDAR_SOVEREIGN: testing::Card = testing::Card {
    name: testing::TestCardName("Felidar Sovereign"),
    definition: || {
        card! {
            cards::creature("Felidar Sovereign", cost![4, W, W], pt!(4, 6));
            keywords::vigilance(),
            keywords::lifelink(),
            abilities::at(
                turns::beginning_of(UPKEEP, YOU).if_(YOU.has_life_at_least(40)),
                effects::win_the_game(),
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;

    use super::FELIDAR_SOVEREIGN;

    #[test]
    fn test_usage() {
        let sovereign = (FELIDAR_SOVEREIGN.definition)();
        assert_eq!(*sovereign.name, "Felidar Sovereign");
        assert_eq!(sovereign.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(sovereign.cost, cost![4, W, W]);
        assert_eq!(sovereign.power_toughness, Some(pt!(4, 6)));
        assert_eq!(
            sovereign.keywords().collect::<Vec<_>>(),
            vec![
                keywords::StaticKeyword::Vigilance,
                keywords::StaticKeyword::Lifelink
            ]
        );
        assert_eq!(
            sovereign.triggered_abilities().copied().collect::<Vec<_>>(),
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(abilities::Condition::LifeAtLeast(YOU, 40)),
                effect: effects::win_the_game(),
            }]
        );
    }
}
