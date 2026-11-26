use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::keywords;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SKY_COURT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Sky Court"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Sky Court", cost![2, U]);
            statics::all(CREATURE).restriction::<events::DeclareBlocker>(
                events::DeclareBlocker::BLOCKER,
                rules::RestrictionId("test/test-sky-court"),
                (!rules::has_keyword(keywords::flying::ID)).of(events::DeclareBlocker::BLOCKER)
                    | rules::has_keyword(keywords::flying::ID).of(events::DeclareBlocker::ATTACKER),
            ),
        }
    },
};
