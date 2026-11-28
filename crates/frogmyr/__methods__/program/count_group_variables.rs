use std::collections::BTreeSet;

use crate::Literal;
use crate::Rule;
use crate::Term;

pub fn count_group_variables(rule: &Rule) -> Vec<&'static str> {
    let Some(Literal::Count { body, .. }) =
        rule.body.iter().find(|literal| matches!(literal, Literal::Count { .. }))
    else {
        return Vec::new();
    };
    let mut body_variables = BTreeSet::new();
    for atom in body {
        for term in &atom.terms {
            if let Term::Var(name) = term {
                body_variables.insert(*name);
            }
        }
    }
    let mut grouped = Vec::new();
    for term in &rule.head.terms {
        if let Term::Var(name) = term
            && body_variables.contains(name)
            && !grouped.contains(name)
        {
            grouped.push(*name);
        }
    }
    grouped
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Literal;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;

    use super::count_group_variables;

    #[test]
    fn test_usage() {
        let rule = Rule {
            head: Atom {
                rel: Rel("counted"),
                terms: vec![Term::Var("P"), Term::Var("N")],
            },
            body: vec![Literal::Count {
                output: "N",
                body: vec![
                    Atom {
                        rel: Rel("member"),
                        terms: vec![Term::Var("C"), Term::Var("P")],
                    },
                    Atom {
                        rel: Rel("tagged"),
                        terms: vec![Term::Var("C")],
                    },
                ],
            }],
        };
        assert_eq!(count_group_variables(&rule), vec!["P"]);
    }
}
