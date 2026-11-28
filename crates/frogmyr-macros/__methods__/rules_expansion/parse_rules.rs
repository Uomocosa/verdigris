use crate::RulesExpansion;
use crate::basic::structs::RuleSpec;

pub fn parse_rules(input: syn::parse::ParseStream) -> syn::Result<Vec<RuleSpec>> {
    let mut rules = Vec::new();
    while !input.is_empty() {
        rules.push(RulesExpansion::parse_rule(input)?);
        if input.is_empty() {
            break;
        }
        input.parse::<syn::Token![;]>()?;
    }
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;

    #[test]
    fn test_usage() {
        let parse = |input: syn::parse::ParseStream| RulesExpansion::parse_rules(input);
        let rules = parse.parse2(quote!(a(X) <- b(X); c(Y) <- d(Y);)).unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].head.rel, "a");
    }
}
