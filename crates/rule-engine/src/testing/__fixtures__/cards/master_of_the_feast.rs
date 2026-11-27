use crate::OPPONENT;
use crate::UPKEEP;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::statics;
use crate::testing;
use crate::turns;

pub const MASTER_OF_THE_FEAST: testing::Card = testing::Card {
    name: testing::TestCardName("Master of the Feast"),
    definition: || {
        card! {
            cards::creature("Master of the Feast", cost![4, B], pt!(5, 5));
            keywords::flying(),
            statics::this().must_attack(),
            abilities::at(
                turns::beginning_of(UPKEEP).of(YOU),
                effects::draw(1).to_player(OPPONENT),
            ),
        }
    },
};
