use quote::quote;

use crate::basic::enums::TermSpec;

pub fn parse_term(input: syn::parse::ParseStream, wildcard: &mut usize) -> syn::Result<TermSpec> {
    if input.peek(syn::Token![_]) {
        input.parse::<syn::Token![_]>()?;
        *wildcard = wildcard.saturating_add(1);
        return Ok(TermSpec::Var(format!("__wildcard_{wildcard}")));
    }
    let negative = input.peek(syn::Token![-]);
    if negative {
        input.parse::<syn::Token![-]>()?;
    }
    if input.peek(syn::LitInt) {
        let literal = input.parse::<syn::LitInt>()?;
        let magnitude = literal.base10_parse::<i64>()?;
        let value = if negative {
            magnitude
                .checked_neg()
                .ok_or_else(|| syn::Error::new_spanned(&literal, "integer is out of range"))?
        } else {
            magnitude
        };
        return Ok(TermSpec::Int(value));
    }
    if negative {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected an integer after `-`",
        ));
    }
    let path = input.parse::<syn::Path>()?;
    if path.segments.len() > 1 {
        return Ok(TermSpec::Const(quote!(#path)));
    }
    let Some(segment) = path.segments.last() else {
        return Err(syn::Error::new_spanned(&path, "expected a term"));
    };
    let name = segment.ident.to_string();
    if name.chars().next().is_some_and(|ch| ch.is_ascii_lowercase()) {
        return Ok(TermSpec::Sym(name));
    }
    if name.chars().count() == 1 || name.chars().any(|ch| ch.is_ascii_lowercase()) {
        return Ok(TermSpec::Var(name));
    }
    Ok(TermSpec::Const(quote!(#path)))
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;
    use crate::basic::enums::TermSpec;

    #[test]
    fn test_usage() {
        let mut wildcard = 0;
        let classify = |tokens| {
            let mut wildcard = 0;
            (|input: syn::parse::ParseStream| RulesExpansion::parse_term(input, &mut wildcard))
                .parse2(tokens)
        };
        assert!(matches!(classify(quote!(Card)).unwrap(), TermSpec::Var(name) if name == "Card"));
        assert!(matches!(classify(quote!(hand)).unwrap(), TermSpec::Sym(name) if name == "hand"));
        assert!(matches!(
            classify(quote!(LAND_CARD)).unwrap(),
            TermSpec::Const(_)
        ));
        assert!(matches!(classify(quote!(-3)).unwrap(), TermSpec::Int(-3)));
        let fresh =
            (|input: syn::parse::ParseStream| RulesExpansion::parse_term(input, &mut wildcard))
                .parse2(quote!(_))
                .unwrap();
        assert!(matches!(fresh, TermSpec::Var(name) if name == "__wildcard_1"));
    }
}
