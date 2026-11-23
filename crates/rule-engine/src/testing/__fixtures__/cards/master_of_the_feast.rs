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
use crate::statics;
use crate::testing;
use crate::turns;

pub const MASTER_OF_THE_FEAST: testing::Card = testing::Card {
    name: testing::TestCardName("Master of the Feast"),
    definition: || {
        card! {
            cards::creature("Master of the Feast", cost![4, B], pt!(5, 5));
            keywords::flying(),
            statics::this().must_attack(),
            abilities::at(
                turns::beginning_of(UPKEEP, YOU),
                effects::draw(1).to_player(OPPONENT),
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::OPPONENT;
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::MASTER_OF_THE_FEAST;

    #[test]
    fn test_usage() {
        let feast = (MASTER_OF_THE_FEAST.definition)();
        assert_eq!(*feast.name, "Master of the Feast");
        assert_eq!(feast.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(feast.cost, cost![4, B]);
        assert_eq!(feast.power_toughness, Some(pt!(5, 5)));
        assert_eq!(
            feast.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Flying]
        );
        assert_eq!(
            feast.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::DeclareAttacker::ID),
                    change: rules::Change::Must,
                }),
            }]
        );
        assert_eq!(
            feast.triggered_abilities().copied().collect::<Vec<_>>(),
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: None,
                effect: effects::draw(1).to_player(OPPONENT),
            }]
        );
    }
}
