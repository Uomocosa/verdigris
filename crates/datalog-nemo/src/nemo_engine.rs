use atomic_delegate_macros::atomic_delegates;
use nemo::datavalues::AnyDataValue;
use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Term;
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
    pub(crate) fn decode_cell(
        cell: &AnyDataValue,
        symbols: &std::collections::BTreeMap<String, &'static str>,
    ) -> Result<Value, EngineError> {
    }
}

impl NemoEngine {
    pub(crate) const ENTITY_IRI_PREFIX: &'static str = "https://verdigris.invalid/entity/";

    pub(crate) fn render_atom_text(atom: &Atom) -> String {
        let terms: Vec<String> = atom.terms.iter().map(Self::render_term_text).collect();
        format!("{}({})", *atom.rel, terms.join(", "))
    }

    pub(crate) fn render_term_text(term: &Term) -> String {
        match term {
            Term::Var(name) => format!("?{name}"),
            Term::Val(Value::Int(number)) => number.to_string(),
            Term::Val(Value::Entity(number)) => {
                format!("<{}{number}>", Self::ENTITY_IRI_PREFIX)
            }
            Term::Val(Value::Sym(text)) => {
                format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;
    use verdigris_datalog_ir::traits::engine::Engine;

    use super::NemoEngine;

    #[test]
    fn test_usage() {
        let engine = NemoEngine;
        let derived = engine.evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
    }

    #[test]
    fn render_atom_text_renders_each_term_shape() {
        assert_eq!(
            NemoEngine::render_atom_text(&Atom {
                rel: Rel("edge"),
                terms: vec![
                    Term::Var("X"),
                    Term::Val(Value::Entity(2)),
                    Term::Val(Value::Sym("ada")),
                ],
            }),
            format!("edge(?X, <{}2>, \"ada\")", NemoEngine::ENTITY_IRI_PREFIX)
        );
    }

    #[test]
    fn render_term_text_renders_each_variant() {
        assert_eq!(NemoEngine::render_term_text(&Term::Var("X")), "?X");
        assert_eq!(NemoEngine::render_term_text(&Term::Val(Value::Int(3))), "3");
    }
}
