use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn dont_lose_unspent_mana(all: &statics::All) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::LoseUnspentMana>(),
            change: rules::Change::Skip,
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

    use super::dont_lose_unspent_mana;

    #[test]
    fn test_usage() {
        let ability = dont_lose_unspent_mana(&statics::all(PLAYER));
        assert_eq!(
            ability.filter,
            effects::Filter::player() & effects::Filter::controlled_by(PLAYER)
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::LoseUnspentMana>(),
                change: rules::Change::Skip,
            }),
        );
    }
}
