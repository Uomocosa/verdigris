use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn restriction<E: Event>(
    all: statics::All,
    requirement: rules::Condition,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::OnlyIf {
                role: rules::Role::Subject,
                condition: requirement,
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
        let ability =
            restriction::<testing::RestrictedRule>(statics::all(CREATURE), rules::Condition::True);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<testing::RestrictedRule>(),
                change: rules::Change::OnlyIf {
                    role: rules::Role::Subject,
                    condition: rules::Condition::True,
                },
            })
        );
    }
}
