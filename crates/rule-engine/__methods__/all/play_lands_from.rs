use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;
use crate::zones;

pub fn play_lands_from(all: statics::All, zone: zones::Zone) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::play_land::FROM_HAND),
            change: rules::Change::Allow {
                holder: events::PlayLand::PLAYER,
                when: rules::in_zone(zone, rules::Role::Player),
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
        let ability = play_lands_from(statics::all(PLAYER), GRAVEYARD);
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(rules::RestrictionId("305.1/from-hand")),
                change: rules::Change::Allow {
                    holder: events::PlayLand::PLAYER,
                    when: rules::in_zone(GRAVEYARD, rules::Role::Player),
                },
            })
        );
    }
}
