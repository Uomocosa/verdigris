use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn attack_without_tapping(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::TapAttacker>(),
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

    use super::attack_without_tapping;

    #[test]
    fn test_usage() {
        let ability = attack_without_tapping(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            }),
        );
    }
}
