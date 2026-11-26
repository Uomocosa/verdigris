use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn cant<E: Event>(
    all: statics::All,
    holder: rules::ArgId,
    id: rules::RestrictionId,
    text: &'static str,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Restrict {
                holder,
                restriction: id.restriction(text, !rules::Condition::True),
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
            "can’t draw",
        );
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Draw>(),
                change: rules::Change::Restrict {
                    holder: events::Draw::PLAYER,
                    restriction: rules::RestrictionId("test/cant")
                        .restriction("can’t draw", !rules::Condition::True,),
                },
            })
        );
    }
}
