use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::statics;
use crate::testing;

pub const RELIQUARY_TOWER: testing::Card = testing::Card {
    name: testing::TestCardName("Reliquary Tower"),
    definition: || {
        card! {
            cards::land("Reliquary Tower", []);
            abilities::tap_add(cards::ManaType::Colorless),
            statics::all(PLAYER.controlled_by(YOU)).no_maximum_hand_size(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::value_rule::ValueRule;

    use super::RELIQUARY_TOWER;

    #[test]
    fn test_usage() {
        let tower = (RELIQUARY_TOWER.definition)();
        assert_eq!(*tower.name, "Reliquary Tower");
        assert_eq!(tower.type_line.types, vec![cards::CardType::Land]);
        assert_eq!(
            tower.activated_abilities().copied().collect::<Vec<_>>(),
            [abilities::tap_add(cards::ManaType::Colorless)]
        );
        assert_eq!(
            tower.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Value(rules::MaximumHandSize::ID),
                    change: rules::Change::NoMaximum,
                }),
            }]
        );
    }
}
