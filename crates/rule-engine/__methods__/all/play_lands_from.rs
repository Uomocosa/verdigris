use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;
use crate::zones;

pub fn play_lands_from(all: &statics::All, zone: zones::Zone) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::play::FROM_HAND),
            change: rules::Change::Allow {
                holder: events::Play::PLAYER.into(),
                when: rules::in_zone(zone, events::Play::PLAYER).of(events::Play::CARD),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::GRAVEYARD;
    use crate::PLAYER;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::play_lands_from;

    #[test]
    fn test_usage() {
        let ability = play_lands_from(&statics::all(PLAYER), GRAVEYARD);
        assert_eq!(
            ability.filter,
            effects::Filter::player() & effects::Filter::controlled_by(PLAYER)
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(rules::RestrictionId("305.1/from-hand")),
                change: rules::Change::Allow {
                    holder: events::Play::PLAYER.into(),
                    when: rules::in_zone(GRAVEYARD, events::Play::PLAYER).of(events::Play::CARD),
                },
            })
        );
    }
}
