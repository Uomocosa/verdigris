use crate::Case;
use crate::traits::engine::Engine;

pub fn check<G: Engine>(engine: &G) -> Result<(), String> {
    for case in Case::cases() {
        if let Err(expected) = case.program.stratify() {
            match engine.evaluate(&case.program, &case.facts) {
                Err(actual) if actual == expected => {}
                Err(actual) => {
                    return Err(format!(
                        "{}: expected rejection {expected:?}, got {actual:?}",
                        case.name
                    ));
                }
                Ok(_) => {
                    return Err(format!(
                        "{}: expected rejection, engine succeeded",
                        case.name
                    ));
                }
            }
        } else {
            let first = engine
                .evaluate(&case.program, &case.facts)
                .map_err(|error| format!("{}: first evaluation failed: {error:?}", case.name))?;
            let second = engine
                .evaluate(&case.program, &case.facts)
                .map_err(|error| format!("{}: second evaluation failed: {error:?}", case.name))?;
            if first != second {
                return Err(format!("{}: evaluations differ", case.name));
            }
            if first != case.expected {
                return Err(format!("{}: derived facts differ from expected", case.name));
            }
        }
    }
    Ok(())
}

// no test_usage necessary
