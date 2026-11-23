use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn skip(all: statics::All, step: rules::StepId) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Skip(step),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::skip;

    #[test]
    fn test_usage() {
        let ability = skip(statics::all(CREATURE), testing::TEST_STEP);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Skip(rules::StepId("test/step"))
        );
    }
}
