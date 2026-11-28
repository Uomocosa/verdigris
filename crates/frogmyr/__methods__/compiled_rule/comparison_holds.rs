use crate::CmpOp;
use crate::Term;
use crate::Value;

use crate::CompiledRule;

pub fn comparison_holds(
    rule: &CompiledRule,
    comparison: &(Term, CmpOp, Term),
    bindings: &[Option<Value>],
) -> bool {
    let (left, op, right) = comparison;
    match (
        rule.substitute(left, bindings),
        rule.substitute(right, bindings),
    ) {
        (Some(first), Some(second)) => match op {
            CmpOp::Eq => first == second,
            CmpOp::Ne => first != second,
            CmpOp::Lt => first < second,
            CmpOp::Le => first <= second,
            CmpOp::Gt => first > second,
            CmpOp::Ge => first >= second,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::CmpOp;
    use crate::Literal;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::comparison_holds;
    use crate::CompiledRule;

    #[test]
    fn test_usage() {
        let rule = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("big"),
                terms: vec![Term::Var("X")],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("num"),
                terms: vec![Term::Var("X")],
            })],
        });
        let comparison = (Term::Var("X"), CmpOp::Gt, Term::Val(Value::Int(3)));
        assert!(comparison_holds(&rule, &comparison, &[Some(Value::Int(4))]));
        assert!(!comparison_holds(
            &rule,
            &comparison,
            &[Some(Value::Int(3))]
        ));
        assert!(!comparison_holds(&rule, &comparison, &[]));
    }
}
