use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn allow(
    all: statics::All,
    restriction: rules::RestrictionId,
    alternative: rules::Condition,
    holder: rules::ArgId,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(restriction),
            change: rules::Change::Allow {
                holder,
                when: alternative,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::allow;

    #[test]
    fn test_usage() {
        let ability = allow(
            statics::all(CREATURE),
            rules::RestrictionId("test/untapped"),
            rules::has::<cards::Flying>().of(testing::TestRule::ATTACKER),
            testing::TestRule::ATTACKER,
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(rules::RestrictionId("test/untapped")),
                change: rules::Change::Allow {
                    holder: testing::TestRule::ATTACKER,
                    when: rules::has::<cards::Flying>().of(testing::TestRule::ATTACKER),
                },
            })
        );
    }
}
