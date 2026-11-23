use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const CHUB_TOAD: testing::Card = testing::Card {
    name: testing::TestCardName("Chub Toad"),
    definition: || {
        card! {
            cards::creature("Chub Toad", cost![2, G], pt!(1, 1));
            abilities::whenever(
                abilities::TriggerEvent::blocks(),
                effects::gets(3, 3).to(THIS),
            ),
            abilities::whenever(
                abilities::TriggerEvent::becomes_blocked(),
                effects::gets(3, 3).to(THIS),
            ),
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

    use super::CHUB_TOAD;

    #[test]
    fn test_usage() {
        let toad = (CHUB_TOAD.definition)();
        assert_eq!(*toad.name, "Chub Toad");
        assert_eq!(toad.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(toad.cost, cost![2, G]);
        assert_eq!(toad.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            toad.triggered_abilities().copied().collect::<Vec<_>>(),
            [
                abilities::TriggeredAbility {
                    event: abilities::TriggerEvent::blocks(),
                    intervening_if: None,
                    effect: effects::gets(3, 3).to(THIS),
                },
                abilities::TriggeredAbility {
                    event: abilities::TriggerEvent::becomes_blocked(),
                    intervening_if: None,
                    effect: effects::gets(3, 3).to(THIS),
                },
            ]
        );
    }
}
