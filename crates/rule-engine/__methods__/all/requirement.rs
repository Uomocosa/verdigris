use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn requirement<E: Event>(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(E::ID),
            change: rules::Change::Must,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::requirement;

    #[test]
    fn test_usage() {
        let ability = requirement::<testing::RestrictedRule>(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::EventId("test/restriction")),
                change: rules::Change::Must,
            })
        );
    }
}
