use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::statics;
use crate::testing;

pub const IRON_GOLEM: testing::Card = testing::Card {
    name: testing::TestCardName("Iron Golem"),
    definition: || {
        card! {
            cards::artifact_creature("Iron Golem", cost![4], pt!(5, 3));
            keywords::vigilance(),
            statics::this().must_attack(),
            statics::this().must_block(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::IRON_GOLEM;

    #[test]
    fn test_usage() {
        let golem = (IRON_GOLEM.definition)();
        assert_eq!(*golem.name, "Iron Golem");
        assert_eq!(
            golem.type_line.types,
            vec![cards::CardType::Artifact, cards::CardType::Creature]
        );
        assert_eq!(golem.cost, cost![4]);
        assert_eq!(golem.power_toughness, Some(pt!(5, 3)));
        assert_eq!(
            golem.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Vigilance]
        );
        assert_eq!(
            golem.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: effects::Filter {
                        object: Some(effects::ObjectRelation::ThisObject),
                        ..CREATURE
                    },
                    change: effects::Change::Rule {
                        question: rules::Question::MustAttack,
                        answer: rules::Answer::MustAttack(true),
                    },
                },
                abilities::Continuous {
                    filter: effects::Filter {
                        object: Some(effects::ObjectRelation::ThisObject),
                        ..CREATURE
                    },
                    change: effects::Change::Demand(rules::DeclareBlocker::ID),
                },
            ]
        );
    }
}
