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
        modification: effects::Modification::Replacement(rules::Replacement::Instead {
            event,
            outcome,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::instead;

    #[test]
    fn test_usage() {
        let ability = instead(
            statics::all(PLAYER.controlled_by(YOU)),
            rules::DRAW_FROM_EMPTY_LIBRARY,
            rules::Outcome::WinTheGame,
        );
        assert_eq!(ability.filter, PLAYER.controlled_by(YOU));
        assert_eq!(
            ability.modification,
            effects::Modification::Replacement(rules::Replacement::Instead {
                event: rules::DRAW_FROM_EMPTY_LIBRARY,
                outcome: rules::Outcome::WinTheGame,
            })
        );
    }
}
