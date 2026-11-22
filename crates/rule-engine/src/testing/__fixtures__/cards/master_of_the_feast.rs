use crate::OPPONENT;
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

pub const MASTER_OF_THE_FEAST: testing::Card = testing::Card {
    name: testing::TestCardName("Master of the Feast"),
    definition: || {
        card! {
            cards::creature("Master of the Feast", cost![4, B], pt!(5, 5));
            keywords::flying(),
            keywords::must_attack_if_able(),
            abilities::at(
                turns::beginning_of(UPKEEP, YOU),
                effects::draw(1).to_player(OPPONENT),
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::OPPONENT;
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;

    use super::MASTER_OF_THE_FEAST;

    #[test]
    fn test_usage() {
        let feast = (MASTER_OF_THE_FEAST.definition)();
        assert_eq!(*feast.name, "Master of the Feast");
        assert_eq!(feast.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(feast.cost, cost![4, B]);
        assert_eq!(feast.power_toughness, Some(pt!(5, 5)));
        assert_eq!(
            feast.keywords,
            vec![
                keywords::Keyword::Flying,
                keywords::Keyword::MustAttackIfAble
            ]
        );
        assert_eq!(
            feast.triggered_abilities,
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: None,
                effect: effects::draw(1).to_player(OPPONENT),
            }]
        );
    }
}
