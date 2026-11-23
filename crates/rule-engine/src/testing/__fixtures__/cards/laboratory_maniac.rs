use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;
use crate::testing;

pub const LABORATORY_MANIAC: testing::Card = testing::Card {
    name: testing::TestCardName("Laboratory Maniac"),
    definition: || {
        card! {
            cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2))
                .subtype(cards::CreatureType::Human.into())
                .subtype(cards::CreatureType::Wizard.into());
            statics::all(PLAYER.controlled_by(YOU)).instead(
                rules::DrawFromEmptyLibrary::ID,
                rules::Outcome::WinTheGame,
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::LABORATORY_MANIAC;

    #[test]
    fn test_usage() {
        let maniac = (LABORATORY_MANIAC.definition)();
        assert_eq!(*maniac.name, "Laboratory Maniac");
        assert_eq!(maniac.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(
            maniac.type_line.subtypes,
            vec![
                cards::Subtype::from(cards::CreatureType::Human),
                cards::Subtype::from(cards::CreatureType::Wizard),
            ]
        );
        assert_eq!(maniac.cost, cost![2, U]);
        assert_eq!(maniac.power_toughness, Some(pt!(2, 2)));
        assert_eq!(
            maniac.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::DrawFromEmptyLibrary::ID),
                    change: rules::Change::Instead(rules::Outcome::WinTheGame),
                }),
            }]
        );
    }
}
