use atomic_delegate_macros::atomic_delegates;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::traits::engine::Engine;

#[derive(Debug, Clone, Copy)]
pub struct DatafrogEngine;

#[rustfmt::skip]
impl Engine for DatafrogEngine {
    fn evaluate(&self, program: &Program, facts: &Facts) -> Result<Facts, EngineError> { Self::evaluate_program(program, facts) }
}

#[atomic_delegates]
impl DatafrogEngine {
    pub(crate) fn evaluate_program(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::traits::engine::Engine;

    use super::DatafrogEngine;

    #[test]
    fn test_usage() {
        let engine = DatafrogEngine;
        let derived = engine.evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
    }
}
