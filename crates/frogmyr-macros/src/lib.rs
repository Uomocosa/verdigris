use proc_macro::TokenStream;

use crate::rules_expansion::RulesExpansion;

#[path = "__basic__/mod.rs"]
mod basic;

mod rules_expansion;

#[path = "../__methods__/mod.rs"]
mod methods;

#[proc_macro]
pub fn rules(input: TokenStream) -> TokenStream {
    let tokens = proc_macro2::TokenStream::from(input);
    RulesExpansion::try_from(tokens)
        .map_or_else(
            |error| error.to_compile_error(),
            |expansion| expansion.into_tokens(),
        )
        .into()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::rules_expansion::RulesExpansion;

    #[test]
    fn test_usage() {
        let expansion = RulesExpansion::try_from(quote!(p(X) <- q(X);)).unwrap();
        assert_eq!(expansion.len(), 1);
    }
}
