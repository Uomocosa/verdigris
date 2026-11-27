use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::rules;
use crate::testing;

pub const TEST_PLAYER_RESTRICTION: testing::Card = testing::Card {
    name: testing::TestCardName("Test Player Restriction"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Restriction", cost![2]);
            abilities::Continuous {
                filter: effects::Filter::player() & effects::Filter::controlled_by(YOU),
                as_long_as: None,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::event::<testing::TestRule>(),
                    change: rules::Change::Restrict {
                        holder: testing::TestRule::PLAYER.into(),
                        restriction: rules::RestrictionId("test/player-restriction").restriction(!rules::Condition::True,
                        ),
                    },
                }),
            },
        }
    },
};
