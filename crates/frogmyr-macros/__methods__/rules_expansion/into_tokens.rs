use proc_macro2::TokenStream;
use quote::quote;

use crate::RulesExpansion;
use crate::basic::enums::CmpOpSpec;
use crate::basic::enums::ConjunctSpec;
use crate::basic::enums::TermSpec;
use crate::basic::structs::AtomSpec;

pub fn into_tokens(expansion: &RulesExpansion) -> TokenStream {
    let term = |spec: &TermSpec| -> TokenStream {
        match spec {
            TermSpec::Var(name) => quote!(::frogmyr::Term::Var(#name)),
            TermSpec::Sym(text) => {
                quote!(::frogmyr::Term::Val(::frogmyr::Value::Sym(#text)))
            }
            TermSpec::Int(value) => {
                quote!(::frogmyr::Term::Val(::frogmyr::Value::Int(#value)))
            }
            TermSpec::Const(expression) => {
                quote!(::frogmyr::Term::Val(::frogmyr::Value::from(#expression)))
            }
        }
    };
    let atom = |spec: &AtomSpec| -> TokenStream {
        let relation = &spec.rel;
        let terms = spec.terms.iter().map(&term);
        quote!(::frogmyr::Atom {
            rel: ::frogmyr::Rel(#relation),
            terms: vec![#(#terms),*],
        })
    };
    let literal = |spec: &ConjunctSpec| -> TokenStream {
        match spec {
            ConjunctSpec::Pos(value) => {
                let value = atom(value);
                quote!(::frogmyr::Literal::Pos(#value))
            }
            ConjunctSpec::Neg(value) => {
                let value = atom(value);
                quote!(::frogmyr::Literal::Neg(#value))
            }
            ConjunctSpec::Count { output, body } => {
                let body = body.iter().map(&atom);
                quote!(::frogmyr::Literal::Count {
                    output: #output,
                    body: vec![#(#body),*],
                })
            }
            ConjunctSpec::Cmp(left, operator, right) => {
                let left = term(left);
                let right = term(right);
                let operator = match operator {
                    CmpOpSpec::Eq => quote!(::frogmyr::CmpOp::Eq),
                    CmpOpSpec::Ne => quote!(::frogmyr::CmpOp::Ne),
                    CmpOpSpec::Lt => quote!(::frogmyr::CmpOp::Lt),
                    CmpOpSpec::Le => quote!(::frogmyr::CmpOp::Le),
                    CmpOpSpec::Gt => quote!(::frogmyr::CmpOp::Gt),
                    CmpOpSpec::Ge => quote!(::frogmyr::CmpOp::Ge),
                };
                quote!(::frogmyr::Literal::Cmp(#left, #operator, #right))
            }
        }
    };
    let rules = expansion.rules.iter().map(|rule| {
        let head = atom(&rule.head);
        let body = rule.body.iter().map(&literal);
        quote!(::frogmyr::Rule {
            head: #head,
            body: vec![#(#body),*],
        })
    });
    quote!(vec![#(#rules),*])
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::RulesExpansion;

    use super::into_tokens;

    #[test]
    fn test_usage() {
        let expansion = RulesExpansion::try_from(quote!(p(N) <- count N: q(X);)).unwrap();
        let tokens = into_tokens(&expansion).to_string();
        assert!(tokens.contains("Count"));
    }
}
