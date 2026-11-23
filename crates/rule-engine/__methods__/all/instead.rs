use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn instead(
    all: statics::All,
    event: rules::EventId,
    outcome: rules::Outcome,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(event),
            change: rules::Change::Instead(outcome),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::instead;

    #[test]
    fn test_usage() {
        let ability = instead(
            statics::all(PLAYER.controlled_by(YOU)),
            rules::DrawFromEmptyLibrary::ID,
            rules::Outcome::WinTheGame,
        );
        assert_eq!(ability.filter, PLAYER.controlled_by(YOU));
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::DrawFromEmptyLibrary::ID),
                change: rules::Change::Instead(rules::Outcome::WinTheGame),
            })
        );
    }
}
