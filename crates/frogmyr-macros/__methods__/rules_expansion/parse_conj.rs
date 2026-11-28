use crate::RulesExpansion;
use crate::basic::enums::ConjunctSpec;

pub fn parse_conj(
    input: syn::parse::ParseStream,
    wildcard: &mut usize,
) -> syn::Result<ConjunctSpec> {
    if input.peek(syn::Token![!]) {
        input.parse::<syn::Token![!]>()?;
        return Ok(ConjunctSpec::Neg(RulesExpansion::parse_atom(
            input, wildcard,
        )?));
    }
    let count = input.fork();
    if count.parse::<syn::Ident>().is_ok_and(|name| name == "count") && count.peek(syn::Ident) {
        input.parse::<syn::Ident>()?;
        let output = input.parse::<syn::Ident>()?.to_string();
        input.parse::<syn::Token![:]>()?;
        let mut body = Vec::new();
        loop {
            body.push(RulesExpansion::parse_atom(input, wildcard)?);
            let follows_atom = {
                let after = input.fork();
                after.parse::<syn::Token![,]>().is_ok()
                    && after.peek(syn::Ident)
                    && after.peek2(syn::token::Paren)
            };
            if !follows_atom {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }
        return Ok(ConjunctSpec::Count { output, body });
    }
    if input.peek(syn::Ident) && input.peek2(syn::token::Paren) {
        return Ok(ConjunctSpec::Pos(RulesExpansion::parse_atom(
            input, wildcard,
        )?));
    }
    let left = RulesExpansion::parse_term(input, wildcard)?;
    let op = RulesExpansion::parse_cmp_op(input)?;
    let right = RulesExpansion::parse_term(input, wildcard)?;
    Ok(ConjunctSpec::Cmp(left, op, right))
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;
    use crate::basic::enums::ConjunctSpec;

    #[test]
    fn test_usage() {
        let mut wildcard = 0;
        let negated =
            (|input: syn::parse::ParseStream| RulesExpansion::parse_conj(input, &mut wildcard))
                .parse2(quote!(!card_type(C, land)))
                .unwrap();
        assert!(matches!(negated, ConjunctSpec::Neg(_)));
        let counted =
            (|input: syn::parse::ParseStream| RulesExpansion::parse_conj(input, &mut wildcard))
                .parse2(quote!(count N: card_type(C, land)))
                .unwrap();
        assert!(matches!(counted, ConjunctSpec::Count { .. }));
    }
}
