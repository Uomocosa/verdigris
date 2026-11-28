use syn::parse::Parser;

use crate::RulesExpansion;

pub fn try_from_token_stream(tokens: proc_macro2::TokenStream) -> syn::Result<RulesExpansion> {
    let rules = RulesExpansion::parse_rules.parse2(tokens)?;
    Ok(RulesExpansion { rules })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::try_from_token_stream;

    #[test]
    fn test_usage() {
        let expansion = try_from_token_stream(quote!(p(X) <- q(X);)).unwrap();
        assert_eq!(expansion.len(), 1);
    }
}
