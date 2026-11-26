use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn restriction<E: Event>(
    all: statics::All,
    holder: impl Into<rules::ArgRef>,
    id: rules::RestrictionId,
    text: &'static str,
    requirement: rules::Condition,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Restrict {
                holder: holder.into(),
                restriction: id.restriction(text, requirement),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::restriction;

    #[test]
    fn test_usage() {
        let ability = restriction::<testing::RestrictedRule>(
            statics::all(CREATURE),
            testing::RestrictedRule::ATTACKER,
            rules::RestrictionId("test/restriction"),
            "attacks only with enough power",
            rules::Condition::True,
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<testing::RestrictedRule>(),
                change: rules::Change::Restrict {
                    holder: testing::RestrictedRule::ATTACKER.into(),
                    restriction: rules::RestrictionId("test/restriction")
                        .restriction("attacks only with enough power", rules::Condition::True,),
                },
            })
        );
    }
}
