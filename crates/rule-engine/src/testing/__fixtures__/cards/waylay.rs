use crate::CLEANUP;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;
use crate::turns;

pub const KNIGHT_TOKEN: testing::Card = testing::Card {
    name: testing::TestCardName("Knight"),
    definition: || {
        card! {
            cards::creature("Knight", cost![W], pt!(2, 2));
            abilities::at(
                turns::beginning_of(CLEANUP, YOU),
                effects::draw(1),
            ),
        }
    },
};

pub const WAYLAY: testing::Card = testing::Card {
    name: testing::TestCardName("Waylay"),
    definition: || {
        cards::sorcery("Waylay", cost![2, W])
            .spell(effects::creates(3, KNIGHT_TOKEN.definition.into()))
    },
};

#[cfg(test)]
mod tests {
    use crate::CLEANUP;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::{KNIGHT_TOKEN, WAYLAY};

    #[test]
    fn test_usage() {
        let knight = (KNIGHT_TOKEN.definition)();
        assert_eq!(*knight.name, "Knight");
        assert_eq!(knight.types, vec![cards::CardType::Creature]);
        assert_eq!(knight.power_toughness, Some(pt!(2, 2)));
        assert_eq!(
            knight.triggered_abilities,
            [abilities::TriggeredAbility {
                event: abilities::TriggerEvent::BeginningOfStep(CLEANUP, YOU),
                intervening_if: None,
                effect: effects::draw(1),
            }]
        );

        let waylay = (WAYLAY.definition)();
        assert_eq!(*waylay.name, "Waylay");
        assert_eq!(waylay.types, vec![cards::CardType::Sorcery]);
        assert_eq!(waylay.cost, cost![2, W]);
        assert_eq!(
            waylay.effect,
            Some(effects::creates(3, KNIGHT_TOKEN.definition.into()))
        );
    }
}
