use crate::TAP_ATTACKERS;
use crate::abilities;
use crate::effects;
use crate::statics;

pub fn tap_when_attacking(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Perform(TAP_ATTACKERS),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::TAP_ATTACKERS;
    use crate::effects;
    use crate::statics;

    use super::tap_when_attacking;

    #[test]
    fn test_usage() {
        let ability = tap_when_attacking(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Perform(TAP_ATTACKERS),
        );
    }
}
