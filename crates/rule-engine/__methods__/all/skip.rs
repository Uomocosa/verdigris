use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn skip<E: Event>(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Skip,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::skip;

    #[test]
    fn test_usage() {
        let ability = skip::<events::Draw>(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Draw>(),
                change: rules::Change::Skip,
            })
        );
    }
}
