use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::basic::enums::CmpOpSpec;
use crate::basic::enums::ConjunctSpec;
use crate::basic::enums::TermSpec;
use crate::basic::structs::AtomSpec;
use crate::basic::structs::RuleSpec;

#[derive(Deref)]
pub struct RulesExpansion {
    pub rules: Vec<RuleSpec>,
}

#[atomic_delegates]
impl TryFrom<proc_macro2::TokenStream> for RulesExpansion {
    type Error = syn::Error;

    fn try_from(tokens: proc_macro2::TokenStream) -> syn::Result<Self> {}
}

#[atomic_delegates]
impl RulesExpansion {
    pub(crate) fn into_tokens(&self) -> proc_macro2::TokenStream {}
    pub(crate) fn parse_rules(input: syn::parse::ParseStream) -> syn::Result<Vec<RuleSpec>> {}
    pub(crate) fn parse_rule(input: syn::parse::ParseStream) -> syn::Result<RuleSpec> {}
    pub(crate) fn parse_conj(
        input: syn::parse::ParseStream,
        wildcard: &mut usize,
    ) -> syn::Result<ConjunctSpec> {
    }
    pub(crate) fn parse_atom(
        input: syn::parse::ParseStream,
        wildcard: &mut usize,
    ) -> syn::Result<AtomSpec> {
    }
    pub(crate) fn parse_term(
        input: syn::parse::ParseStream,
        wildcard: &mut usize,
    ) -> syn::Result<TermSpec> {
    }
    pub(crate) fn parse_cmp_op(input: syn::parse::ParseStream) -> syn::Result<CmpOpSpec> {}
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::RulesExpansion;

    #[test]
    fn test_usage() {
        let expansion = RulesExpansion::try_from(quote! {
            p(X) <- q(X), X > 3;
        })
        .unwrap();
        let tokens = expansion.into_tokens().to_string();
        assert!(tokens.contains("frogmyr"));
        assert!(tokens.contains("Cmp"));
    }
}
