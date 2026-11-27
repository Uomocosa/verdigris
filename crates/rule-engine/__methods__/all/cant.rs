use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn cant<E: Event>(
    all: statics::All,
    holder: impl Into<rules::ArgRef>,
    id: rules::RestrictionId,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Restrict {
                holder: holder.into(),
                restriction: id.restriction(!rules::Condition::True),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::cant;

    #[test]
    fn test_usage() {
        let ability = cant::<events::Draw>(
            statics::all(PLAYER),
            events::Draw::PLAYER,
            rules::RestrictionId("test/cant"),
        );
        assert_eq!(ability.filter, effects::Filter::player(PLAYER));
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Draw>(),
                change: rules::Change::Restrict {
                    holder: events::Draw::PLAYER.into(),
                    restriction: rules::RestrictionId("test/cant")
                        .restriction(!rules::Condition::True),
                },
            })
        );
    }
}
