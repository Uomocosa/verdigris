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
