use crate::testing::prelude::*;

pub const TEST_PLAYER_RESTRICTION: Card = Card {
    name: TestCardName("Test Player Restriction"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Restriction", cost![2]);
            Continuous {
                filter: player().and(controlled_by(YOU)),
                as_long_as: None,
                modification: Modification::Rule(RuleChange {
                    target: Target::event::<TestRule>(),
                    change: Change::Restrict {
                        holder: TestRule::PLAYER.into(),
                        restriction: RestrictionId("test/player-restriction").restriction(!Condition::True,
                        ),
                    },
                }),
            },
        }
    },
};
