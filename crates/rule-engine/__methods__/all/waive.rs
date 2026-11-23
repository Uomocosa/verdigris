use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn waive<Ctx>(all: statics::All, clause: rules::ClauseId<Ctx>) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Waive(clause.key),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::waive;

    #[test]
    fn test_usage() {
        let ability = waive(
            statics::all(CREATURE),
            rules::ClauseId::<rules::AttackCtx>::new("test/untapped"),
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Waive(rules::ClauseKey("test/untapped"))
        );
    }
}
