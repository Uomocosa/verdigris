use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn dont_lose_unspent_mana(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(events::LoseUnspentMana::ID),
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
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::dont_lose_unspent_mana;

    #[test]
    fn test_usage() {
        let ability = dont_lose_unspent_mana(statics::all(PLAYER));
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(events::LoseUnspentMana::ID),
                change: rules::Change::Skip,
            }),
        );
    }
}
