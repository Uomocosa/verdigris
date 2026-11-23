use crate::OPPONENT;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::statics;
use crate::testing;

pub const PLATINUM_ANGEL: testing::Card = testing::Card {
    name: testing::TestCardName("Platinum Angel"),
    definition: || {
        card! {
            cards::artifact_creature("Platinum Angel", cost![7], pt!(4, 4))
                .subtype(cards::CreatureType::Angel.into());
            keywords::flying(),
            statics::all(PLAYER.controlled_by(YOU)).cant_lose_the_game(),
            statics::all(PLAYER.controlled_by(OPPONENT)).cant_win_the_game(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::OPPONENT;
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::PLATINUM_ANGEL;

    #[test]
    fn test_usage() {
        let angel = (PLATINUM_ANGEL.definition)();
        assert_eq!(*angel.name, "Platinum Angel");
        assert_eq!(
            angel.type_line.types,
            vec![cards::CardType::Artifact, cards::CardType::Creature]
        );
        assert_eq!(angel.type_line.supertypes, Vec::<cards::Supertype>::new());
        assert_eq!(
            angel.type_line.subtypes,
            vec![cards::Subtype::from(cards::CreatureType::Angel)]
        );
        assert_eq!(angel.cost, cost![7]);
        assert_eq!(angel.power_toughness, Some(pt!(4, 4)));
        assert_eq!(
            angel.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Flying]
        );
        assert_eq!(
            angel.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::LosesTheGame::ID),
                        change: rules::Change::Cant,
                    }),
                },
                abilities::Continuous {
                    filter: PLAYER.controlled_by(OPPONENT),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::WinsTheGame::ID),
                        change: rules::Change::Cant,
                    }),
                },
            ]
        );
    }
}
