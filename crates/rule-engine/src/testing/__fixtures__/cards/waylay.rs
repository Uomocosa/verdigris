use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const KNIGHT_TOKEN: testing::Card = testing::Card {
    name: testing::TestCardName("Knight"),
    definition: || {
        card! {
            cards::creature("Knight", cost![W], pt!(2, 2));
            abilities::at(
                abilities::TriggerEvent::beginning_of_next_cleanup(),
                effects::exile(),
            ),
        }
    },
};

pub const WAYLAY: testing::Card = testing::Card {
    name: testing::TestCardName("Waylay"),
    definition: || {
        cards::instant("Waylay", cost![2, W])
            .spell(effects::creates(3, KNIGHT_TOKEN.definition.into()))
    },
};
