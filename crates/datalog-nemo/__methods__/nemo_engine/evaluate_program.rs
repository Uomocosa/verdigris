use std::collections::BTreeMap;

use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Literal;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;

use crate::NemoEngine;

pub fn evaluate_program(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    let _strata = program.stratify()?;
    let mut names: BTreeMap<String, Rel> = BTreeMap::new();
    let mut symbols: BTreeMap<String, &'static str> = BTreeMap::new();
    let mut collect_term = |term: &Term| {
        if let Term::Val(Value::Sym(text)) = term {
            symbols.insert(text.to_string(), text);
        }
    };
    for rule in &program.rules {
        names.insert(rule.head.rel.to_string(), rule.head.rel);
        for term in &rule.head.terms {
            collect_term(term);
        }
        for literal in &rule.body {
            let (Literal::Pos(atom) | Literal::Neg(atom)) = literal else {
                continue;
            };
            for term in &atom.terms {
                collect_term(term);
            }
        }
    }
    for (_, tuple) in facts.iter() {
        for value in tuple {
            if let Value::Sym(text) = value {
                symbols.insert(text.to_string(), text);
            }
        }
    }
    let text = NemoEngine::render_program_text(program, facts);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| EngineError::Backend(error.to_string()))?;
    runtime.block_on(async {
        let mut engine = nemo::api::load_string(text)
            .await
            .map_err(|error| EngineError::Backend(error.to_string()))?;
        nemo::api::reason(&mut engine)
            .await
            .map_err(|error| EngineError::Backend(error.to_string()))?;
        let mut derived = facts.clone();
        for predicate in nemo::api::output_predicates(&engine) {
            let name = predicate.to_string();
            let Some(relation) = names.get(&name) else {
                return Err(EngineError::Backend(format!(
                    "unknown output predicate {name}"
                )));
            };
            let Some(rows) = engine
                .predicate_rows(&predicate)
                .await
                .map_err(|error| EngineError::Backend(error.to_string()))?
            else {
                continue;
            };
            for row in rows {
                let mut tuple = Vec::with_capacity(row.len());
                for cell in &row {
                    tuple.push(NemoEngine::decode_cell(cell, &symbols)?);
                }
                let _ = derived.add(*relation, tuple);
            }
        }
        Ok(derived)
    })
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::evaluate_program;
    use crate::NemoEngine;
    use verdigris_datalog_ir::traits::engine::Engine;

    #[test]
    fn test_usage() {
        let program = Program {
            rules: vec![Rule {
                head: Atom {
                    rel: Rel("path"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                },
                body: vec![Literal::Pos(Atom {
                    rel: Rel("edge"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                })],
            }],
        };
        let mut facts = Facts::default();
        assert!(facts.add(Rel("edge"), vec![Value::Int(1), Value::Int(2)]));
        let derived = evaluate_program(&program, &facts).unwrap();
        assert!(derived.contains(&Rel("path"), &[Value::Int(1), Value::Int(2)]));
        assert!(derived.contains(&Rel("edge"), &[Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn shared_conformance_cases_pass() {
        let engine = NemoEngine;
        let started = std::time::Instant::now();
        let outcome = verdigris_datalog_ir::Case::check(&engine);
        let elapsed = started.elapsed();
        println!(
            "shared conformance: {outcome:?} in {}ms",
            elapsed.as_millis()
        );
        assert!(outcome.is_ok());
        let repeated = verdigris_datalog_ir::Case::entity_traversal();
        for _ in 0..1000 {
            let derived = engine.evaluate(&repeated.program, &repeated.facts).unwrap();
            assert_eq!(derived, repeated.expected);
        }
    }
}
