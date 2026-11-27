use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn attack_as_though_it_had_haste(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::declare_attacker::SUMMONING_SICK),
            change: rules::Change::AsThough {
                holder: events::DeclareAttacker::ATTACKER.into(),
            },
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

    use super::attack_as_though_it_had_haste;

    #[test]
    fn test_usage() {
        let ability = attack_as_though_it_had_haste(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::declare_attacker::SUMMONING_SICK),
                change: rules::Change::AsThough {
                    holder: events::DeclareAttacker::ATTACKER.into(),
                },
            })
        );
    }
}
