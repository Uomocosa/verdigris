use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn tap_when_attacking(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(rules::TapAttacker::ID),
            change: rules::Change::Perform,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::tap_when_attacking;

    #[test]
    fn test_usage() {
        let ability = tap_when_attacking(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::TapAttacker::ID),
                change: rules::Change::Perform,
            }),
        );
    }
}
