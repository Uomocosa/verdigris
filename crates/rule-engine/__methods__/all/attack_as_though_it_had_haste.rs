use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn attack_as_though_it_had_haste(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Waive(rules::declare_attacker::SUMMONING_SICK.key),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::attack_as_though_it_had_haste;

    #[test]
    fn test_usage() {
        let ability = attack_as_though_it_had_haste(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Waive(rules::declare_attacker::SUMMONING_SICK.key)
        );
    }
}
