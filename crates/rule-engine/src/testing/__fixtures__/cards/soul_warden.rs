use crate::ANOTHER_CREATURE;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SOUL_WARDEN: testing::Card = testing::Card {
    name: testing::TestCardName("Soul Warden"),
    definition: || {
        card! {
            cards::creature("Soul Warden", cost![W], pt!(1, 1));
            abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU)),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::SOUL_WARDEN;

    #[test]
    fn test_usage() {
        let warden = (SOUL_WARDEN.definition)();
        assert_eq!(*warden.name, "Soul Warden");
        assert_eq!(warden.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(warden.cost, cost![W]);
        assert_eq!(warden.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            warden.triggered_abilities,
            [abilities::TriggeredAbility {
                event: ANOTHER_CREATURE.enters(),
                intervening_if: None,
                effect: effects::gain_life(1).to_player(YOU),
            }]
        );
    }
}
