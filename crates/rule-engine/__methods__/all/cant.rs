use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn cant<E: Event>(all: statics::All, role: rules::Role) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(E::ID),
            change: rules::Change::Cant { role },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::cant;

    #[test]
    fn test_usage() {
        let ability = cant::<events::Draw>(statics::all(PLAYER), rules::Role::Player);
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(events::Draw::ID),
                change: rules::Change::Cant {
                    role: rules::Role::Player,
                },
            })
        );
    }
}
