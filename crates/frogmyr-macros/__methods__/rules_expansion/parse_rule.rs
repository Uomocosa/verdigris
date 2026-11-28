use crate::RulesExpansion;
use crate::basic::structs::RuleSpec;

pub fn parse_rule(input: syn::parse::ParseStream) -> syn::Result<RuleSpec> {
    let mut wildcard = 0;
    let head = RulesExpansion::parse_atom(input, &mut wildcard)?;
    input.parse::<syn::Token![<-]>()?;
    let mut body = Vec::new();
    loop {
        body.push(RulesExpansion::parse_conj(input, &mut wildcard)?);
        if input.is_empty() || input.peek(syn::Token![;]) {
            break;
        }
        input.parse::<syn::Token![,]>()?;
    }
    Ok(RuleSpec { head, body })
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;

    #[test]
    fn test_usage() {
        let parse = |input: syn::parse::ParseStream| RulesExpansion::parse_rule(input);
        let rule = parse.parse2(quote!(p(X) <- q(X), X > 3)).unwrap();
        assert_eq!(rule.head.rel, "p");
        assert_eq!(rule.body.len(), 2);
    }
}
