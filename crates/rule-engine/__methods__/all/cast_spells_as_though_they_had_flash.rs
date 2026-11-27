use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn cast_spells_as_though_they_had_flash(all: &statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::cast::TIMING),
            change: rules::Change::AsThough {
                holder: events::Cast::SPELL.controller(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::cast_spells_as_though_they_had_flash;

    #[test]
    fn test_usage() {
        let ability = cast_spells_as_though_they_had_flash(&statics::you());
        assert_eq!(
            ability.filter,
            effects::Filter::player() & effects::Filter::controlled_by(YOU)
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::cast::TIMING),
                change: rules::Change::AsThough {
                    holder: events::Cast::SPELL.controller(),
                },
            })
        );
    }
}
