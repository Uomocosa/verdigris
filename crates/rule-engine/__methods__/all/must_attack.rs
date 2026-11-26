use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn must_attack(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::DeclareAttacker>(),
            change: rules::Change::Must,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::must_attack;

    #[test]
    fn test_usage() {
        let ability = must_attack(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareAttacker>(),
                change: rules::Change::Must,
            })
        );
        assert_eq!(
            events::DeclareAttacker::ID,
            rules::EventId("508.1a/declare-attacker")
        );
    }
}
