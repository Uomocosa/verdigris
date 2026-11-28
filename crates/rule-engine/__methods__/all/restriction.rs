use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn restriction<E: Event>(
    all: &statics::All,
    holder: impl Into<rules::ArgRef>,
    requirement: rules::Condition,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Restrict {
                holder: holder.into(),
                restriction: rules::RestrictionId {
                    card: rules::RestrictionId::UNSET,
                    kind: "restriction",
                    event: E::ID,
                }
                .restriction(requirement),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;
    use crate::testing;

    use super::restriction;

    #[test]
    fn test_usage() {
        let ability = restriction::<testing::RestrictedRule>(
            &statics::all(creature()),
            testing::RestrictedRule::ATTACKER,
            rules::Condition::True,
        );
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<testing::RestrictedRule>(),
                change: rules::Change::Restrict {
                    holder: testing::RestrictedRule::ATTACKER.into(),
                    restriction: rules::RestrictionId {
                        card: rules::RestrictionId::UNSET,
                        kind: "restriction",
                        event: testing::RestrictedRule::ID,
                    }
                    .restriction(rules::Condition::True),
                },
            })
        );
    }
}
