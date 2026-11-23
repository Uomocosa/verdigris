use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::value_rule::ValueRule;
use crate::statics;

pub fn play_additional_lands(all: statics::All, count: u32) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange::Value {
            rule: rules::LandPlays::ID,
            change: rules::ValueChange::Increase(count),
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

    use super::play_additional_lands;

    #[test]
    fn test_usage() {
        let ability = play_additional_lands(statics::all(PLAYER), 2);
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange::Value {
                rule: rules::LandPlays::ID,
                change: rules::ValueChange::Increase(2),
            })
        );
    }
}
