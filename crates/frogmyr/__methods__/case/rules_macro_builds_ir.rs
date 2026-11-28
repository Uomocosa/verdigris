use crate::Atom;
use crate::Case;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;
use crate::rules;

const LAND_CARD: u64 = 13;

pub fn rules_macro_builds_ir() -> Case {
    let comparison = Case::comparison_literal();
    let from_macro: Vec<Rule> = rules![
        big(X) <- num(X), X > 3;
        dead(1) <- 1 > 2;
        pair_count(N) <- count N: num(X);
        small(X) <- num(X), !big(X);
        marked(LAND_CARD) <- num(4);
    ];
    let mut hand_built = comparison.program.rules.clone();
    hand_built.push(Rule {
        head: Atom {
            rel: Rel("pair_count"),
            terms: vec![Term::Var("N")],
        },
        body: vec![Literal::Count {
            output: "N",
            body: vec![Atom {
                rel: Rel("num"),
                terms: vec![Term::Var("X")],
            }],
        }],
    });
    hand_built.push(Rule {
        head: Atom {
            rel: Rel("small"),
            terms: vec![Term::Var("X")],
        },
        body: vec![
            Literal::Pos(Atom {
                rel: Rel("num"),
                terms: vec![Term::Var("X")],
            }),
            Literal::Neg(Atom {
                rel: Rel("big"),
                terms: vec![Term::Var("X")],
            }),
        ],
    });
    hand_built.push(Rule {
        head: Atom {
            rel: Rel("marked"),
            terms: vec![Term::Val(Value::Entity(LAND_CARD))],
        },
        body: vec![Literal::Pos(Atom {
            rel: Rel("num"),
            terms: vec![Term::Val(Value::Int(4))],
        })],
    });
    assert_eq!(from_macro, hand_built);
    let mut expected = comparison.expected.clone();
    let _ = expected.add(Rel("pair_count"), vec![Value::Int(5)]);
    let _ = expected.add(Rel("small"), vec![Value::Int(1)]);
    let _ = expected.add(Rel("small"), vec![Value::Int(2)]);
    let _ = expected.add(Rel("small"), vec![Value::Int(3)]);
    let _ = expected.add(Rel("marked"), vec![Value::Entity(LAND_CARD)]);
    Case {
        name: "rules_macro_builds_ir",
        program: Program { rules: from_macro },
        facts: comparison.facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::rules_macro_builds_ir;

    #[test]
    fn test_usage() {
        let case = rules_macro_builds_ir();
        assert_eq!(case.name, "rules_macro_builds_ir");
        assert_eq!(case.program.rules.len(), 5);
    }
}
