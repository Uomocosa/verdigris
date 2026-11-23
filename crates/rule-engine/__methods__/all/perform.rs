use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn perform(all: statics::All, step: rules::StepId) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Perform(step),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::perform;

    #[test]
    fn test_usage() {
        let ability = perform(statics::all(CREATURE), testing::TEST_STEP);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Perform(rules::StepId("test/step"))
        );
    }
}
