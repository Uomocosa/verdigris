use atomic_delegate_macros::atomic_delegates;
use nemo::datavalues::AnyDataValue;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Value;
use verdigris_datalog_ir::traits::engine::Engine;

#[derive(Debug, Clone, Copy)]
pub struct NemoEngine;

#[rustfmt::skip]
impl Engine for NemoEngine {
    fn evaluate(&self, program: &Program, facts: &Facts) -> Result<Facts, EngineError> { Self::evaluate_program(program, facts) }
}

#[atomic_delegates]
impl NemoEngine {
    pub(crate) fn evaluate_program(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    }
    pub(crate) fn render_program_text(program: &Program, facts: &Facts) -> String {}
    pub(crate) fn decode_cell(cell: &AnyDataValue) -> Result<Value, EngineError> {}
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::traits::engine::Engine;

    use super::NemoEngine;

    #[test]
    fn test_usage() {
        let engine = NemoEngine;
        let derived = engine.evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
    }
}
