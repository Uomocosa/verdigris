use crate::TAP_ATTACKERS;
use crate::abilities;
use crate::effects;
use crate::statics;

pub fn attack_without_tapping(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Skip(TAP_ATTACKERS),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::TAP_ATTACKERS;
    use crate::effects;
    use crate::statics;

    use super::attack_without_tapping;

    #[test]
    fn test_usage() {
        let ability = attack_without_tapping(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(ability.change, effects::Change::Skip(TAP_ATTACKERS),);
    }
}
