use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn no_maximum_hand_size(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::value::<rules::MaximumHandSize>(),
            change: rules::Change::NoMaximum,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::no_maximum_hand_size;

    #[test]
    fn test_usage() {
        let ability = no_maximum_hand_size(statics::all(PLAYER));
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::value::<rules::MaximumHandSize>(),
                change: rules::Change::NoMaximum,
            })
        );
    }
}
