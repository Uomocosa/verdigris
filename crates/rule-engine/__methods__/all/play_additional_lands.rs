use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn play_additional_lands(all: &statics::All, count: u32) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::value::<rules::LandPlays>(),
            change: rules::Change::Increase(count),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::play_additional_lands;

    #[test]
    fn test_usage() {
        let ability = play_additional_lands(&statics::all(PLAYER), 2);
        assert_eq!(
            ability.filter,
            effects::Filter::player() & effects::Filter::controlled_by(PLAYER)
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::value::<rules::LandPlays>(),
                change: rules::Change::Increase(2),
            })
        );
    }
}
