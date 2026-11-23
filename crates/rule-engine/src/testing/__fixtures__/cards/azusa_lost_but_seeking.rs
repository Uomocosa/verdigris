use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::statics;
use crate::testing;

pub const AZUSA_LOST_BUT_SEEKING: testing::Card = testing::Card {
    name: testing::TestCardName("Azusa, Lost but Seeking"),
    definition: || {
        card! {
            cards::creature("Azusa, Lost but Seeking", cost![2, G], pt!(1, 2))
                .supertype(cards::Supertype::Legendary);
            statics::all(PLAYER.controlled_by(YOU)).play_additional_lands(2),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::value_rule::ValueRule;

    use super::AZUSA_LOST_BUT_SEEKING;

    #[test]
    fn test_usage() {
        let azusa = (AZUSA_LOST_BUT_SEEKING.definition)();
        assert_eq!(*azusa.name, "Azusa, Lost but Seeking");
        assert_eq!(
            azusa.type_line.supertypes,
            vec![cards::Supertype::Legendary]
        );
        assert_eq!(azusa.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(azusa.cost, cost![2, G]);
        assert_eq!(azusa.power_toughness, Some(pt!(1, 2)));
        assert_eq!(
            azusa.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange::Value {
                    rule: rules::LandPlays::ID,
                    change: rules::ValueChange::Increase(2),
                }),
            }]
        );
    }
}
