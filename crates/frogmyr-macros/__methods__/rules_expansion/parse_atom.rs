use crate::RulesExpansion;
use crate::basic::structs::AtomSpec;

pub fn parse_atom(input: syn::parse::ParseStream, wildcard: &mut usize) -> syn::Result<AtomSpec> {
    let relation = input.parse::<syn::Ident>()?;
    let name = relation.to_string();
    if !name.chars().next().is_some_and(|ch| ch.is_ascii_lowercase()) {
        return Err(syn::Error::new(
            relation.span(),
            "a relation name must start with a lowercase letter",
        ));
    }
    let content;
    let _parentheses = syn::parenthesized!(content in input);
    let mut terms = Vec::new();
    while !content.is_empty() {
        terms.push(RulesExpansion::parse_term(&content, wildcard)?);
        if content.is_empty() {
            break;
        }
        content.parse::<syn::Token![,]>()?;
    }
    Ok(AtomSpec { rel: name, terms })
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;

    #[test]
    fn test_usage() {
        let mut wildcard = 0;
        let parser =
            |input: syn::parse::ParseStream| RulesExpansion::parse_atom(input, &mut wildcard);
        let atom = parser.parse2(quote!(in_zone(C, hand, P))).unwrap();
        assert_eq!(atom.rel, "in_zone");
        assert_eq!(atom.terms.len(), 3);
    }
}
