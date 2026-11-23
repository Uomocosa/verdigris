use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SCROLL_THIEF: testing::Card = testing::Card {
    name: testing::TestCardName("Scroll Thief"),
    definition: || {
        card! {
            cards::creature("Scroll Thief", cost![2, U], pt!(1, 3));
            abilities::whenever(
                abilities::TriggerEvent::dealt_combat_damage_to_player(),
                effects::draw(1),
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::SCROLL_THIEF;

    #[test]
    fn test_usage() {
        let thief = (SCROLL_THIEF.definition)();
        assert_eq!(*thief.name, "Scroll Thief");
        assert_eq!(thief.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(thief.cost, cost![2, U]);
        assert_eq!(thief.power_toughness, Some(pt!(1, 3)));
        assert_eq!(
            thief.triggered_abilities().copied().collect::<Vec<_>>(),
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::dealt_combat_damage_to_player(),
                intervening_if: None,
                effect: effects::draw(1),
            }]
        );
    }
}
