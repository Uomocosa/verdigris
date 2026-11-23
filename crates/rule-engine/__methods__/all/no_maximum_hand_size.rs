use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::value_rule::ValueRule;
use crate::statics;

pub fn no_maximum_hand_size(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange::Value {
            rule: rules::MaximumHandSize::ID,
            change: rules::ValueChange::NoMaximum,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::value_rule::ValueRule;
    use crate::statics;

    use super::no_maximum_hand_size;

    #[test]
    fn test_usage() {
        let ability = no_maximum_hand_size(statics::all(PLAYER));
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange::Value {
                rule: rules::MaximumHandSize::ID,
                change: rules::ValueChange::NoMaximum,
            })
        );
    }
}
