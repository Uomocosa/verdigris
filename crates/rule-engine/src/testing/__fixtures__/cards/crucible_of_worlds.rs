use crate::GRAVEYARD;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const CRUCIBLE_OF_WORLDS: testing::Card = testing::Card {
    name: testing::TestCardName("Crucible of Worlds"),
    definition: || {
        card! {
            cards::CardDefinition {
                name: "Crucible of Worlds".into(),
                cost: cost![3],
                type_line: cards::TypeLine {
                    types: vec![cards::CardType::Artifact],
                    ..Default::default()
                },
                power_toughness: None,
                text: Vec::new(),
            };
            statics::all(PLAYER.controlled_by(YOU)).play_lands_from(GRAVEYARD),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::GRAVEYARD;
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;

    use super::CRUCIBLE_OF_WORLDS;

    #[test]
    fn test_usage() {
        let crucible = (CRUCIBLE_OF_WORLDS.definition)();
        assert_eq!(*crucible.name, "Crucible of Worlds");
        assert_eq!(crucible.type_line.types, vec![cards::CardType::Artifact]);
        assert_eq!(crucible.cost, cost![3]);
        assert_eq!(crucible.colors(), vec![]);
        assert_eq!(
            crucible.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange::Allow(
                    rules::play_land::FROM_HAND.key,
                    rules::in_zone(GRAVEYARD, rules::Role::Player),
                )),
            }]
        );
    }
}
