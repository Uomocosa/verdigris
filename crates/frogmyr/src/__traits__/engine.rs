use crate::EngineError;
use crate::Facts;
use crate::Program;

pub trait Engine {
    fn evaluate(&self, program: &Program, facts: &Facts) -> Result<Facts, EngineError>;
}

// no test_usage necessary
