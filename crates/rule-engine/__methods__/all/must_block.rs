use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn must_block(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::DeclareBlocker>(),
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

    use super::must_block;

    #[test]
    fn test_usage() {
        let ability = must_block(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareBlocker>(),
                change: rules::Change::Must,
            })
        );
        assert_eq!(
            events::DeclareBlocker::ID,
            rules::EventId("509.1a/declare-blocker")
        );
    }
}
