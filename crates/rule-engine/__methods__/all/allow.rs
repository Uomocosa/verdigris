use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn allow(
    all: &statics::All,
    restriction: rules::RestrictionId,
    alternative: rules::Condition,
    holder: impl Into<rules::ArgRef>,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(restriction),
            change: rules::Change::Allow {
                holder: holder.into(),
                when: alternative,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::keywords;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::allow;

    #[test]
    fn test_usage() {
        let ability = allow(
            &statics::all(creature()),
            testing::UNTAPPED,
            rules::has_keyword(keywords::flying::ID).of(testing::TestRule::ATTACKER),
            testing::TestRule::ATTACKER,
        );
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(testing::UNTAPPED),
                change: rules::Change::Allow {
                    holder: testing::TestRule::ATTACKER.into(),
                    when: rules::has_keyword(keywords::flying::ID).of(testing::TestRule::ATTACKER),
                },
            })
        );
    }
}
