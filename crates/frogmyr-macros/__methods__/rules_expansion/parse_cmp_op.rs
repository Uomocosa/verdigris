use crate::basic::enums::CmpOpSpec;

pub fn parse_cmp_op(input: syn::parse::ParseStream) -> syn::Result<CmpOpSpec> {
    if input.peek(syn::Token![<=]) {
        input.parse::<syn::Token![<=]>()?;
        return Ok(CmpOpSpec::Le);
    }
    if input.peek(syn::Token![>=]) {
        input.parse::<syn::Token![>=]>()?;
        return Ok(CmpOpSpec::Ge);
    }
    if input.peek(syn::Token![==]) {
        input.parse::<syn::Token![==]>()?;
        return Ok(CmpOpSpec::Eq);
    }
    if input.peek(syn::Token![!=]) {
        input.parse::<syn::Token![!=]>()?;
        return Ok(CmpOpSpec::Ne);
    }
    if input.peek(syn::Token![<]) {
        input.parse::<syn::Token![<]>()?;
        return Ok(CmpOpSpec::Lt);
    }
    if input.peek(syn::Token![>]) {
        input.parse::<syn::Token![>]>()?;
        return Ok(CmpOpSpec::Gt);
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "expected a comparison operator",
    ))
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse::Parser;

    use crate::RulesExpansion;
    use crate::basic::enums::CmpOpSpec;

    #[test]
    fn test_usage() {
        let operator = RulesExpansion::parse_cmp_op.parse2(quote!(>=)).unwrap();
        assert!(matches!(operator, CmpOpSpec::Ge));
        assert!(RulesExpansion::parse_cmp_op.parse2(quote!(+)).is_err());
    }
}
