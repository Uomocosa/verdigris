use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::testing;

pub const TEST_PLAYER_RESTRICTION: testing::Card = testing::Card {
    name: testing::TestCardName("Test Player Restriction"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Restriction", cost![2]);
            abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(testing::TestRule::ID),
                    change: rules::Change::OnlyIf {
                        role: rules::Role::Player,
                        condition: !rules::Predicate::True,
                    },
                }),
            },
        }
    },
};
