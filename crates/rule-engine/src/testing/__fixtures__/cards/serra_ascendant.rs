use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const SERRA_ASCENDANT: testing::Card = testing::Card {
    name: testing::TestCardName("Serra Ascendant"),
    definition: || {
        card! {
            cards::creature("Serra Ascendant", cost![W], pt!(1, 1));
            keywords::lifelink(),
            statics::this().get_pt(5, 5).as_long_as(rules::life().at_least(30).of(rules::YOU)),
            statics::this().gain(keywords::flying()).as_long_as(rules::life().at_least(30).of(rules::YOU)),
        }
    },
};
