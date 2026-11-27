use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;

#[proc_macro_derive(EventArgs, attributes(arg))]
pub fn derive_event_args(input: TokenStream) -> TokenStream {
    EventArgsExpansion::expanded(input.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}

pub(crate) struct EventArgsExpansion {
    name: syn::Ident,
    fields: Vec<(syn::Ident, syn::Type)>,
    all_arg: bool,
}

impl TryFrom<syn::DeriveInput> for EventArgsExpansion {
    type Error = syn::Error;

    fn try_from(input: syn::DeriveInput) -> Result<Self, Self::Error> {
        let syn::Data::Struct(data) = input.data else {
            return Err(syn::Error::new(
                input.ident.span(),
                "`#[derive(EventArgs)]` needs a struct",
            ));
        };
        let syn::Fields::Named(named) = data.fields else {
            return Err(syn::Error::new(
                input.ident.span(),
                "`#[derive(EventArgs)]` needs named fields",
            ));
        };
        let mut fields = Vec::new();
        let total = named.named.len();
        for field in named.named {
            if !field.attrs.iter().any(|attr| attr.path().is_ident("arg")) {
                continue;
            }
            let Some(name) = field.ident else { continue };
            fields.push((name, field.ty));
        }
        let all_arg = fields.len() == total;
        Ok(Self {
            name: input.ident,
            fields,
            all_arg,
        })
    }
}

impl EventArgsExpansion {
    pub(crate) fn expanded(
        tokens: proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let input: syn::DeriveInput = syn::parse2(tokens)?;
        Self::try_from(input)?.into_tokens()
    }

    pub(crate) fn const_name(field: &syn::Ident) -> syn::Ident {
        let mut upper = String::with_capacity(field.to_string().len());
        for (index, ch) in field.to_string().chars().enumerate() {
            if index > 0 && ch.is_ascii_uppercase() {
                upper.push('_');
            }
            upper.push(ch.to_ascii_uppercase());
        }
        format_ident!("{upper}", span = field.span())
    }

    pub(crate) fn into_tokens(self) -> syn::Result<proc_macro2::TokenStream> {
        let name = &self.name;
        let consts = self.fields.iter().map(|(field, _)| {
            let upper = Self::const_name(field);
            let text = field.to_string();
            quote!(pub const #upper: crate::rules::ArgId = crate::rules::ArgId(#text);)
        });
        let uppers = self.fields.iter().map(|(field, _)| Self::const_name(field));
        let reads = self.fields.iter().map(|(field, _)| {
            let text = field.to_string();
            quote!(#text => Some(crate::rules::ArgValue::from(self.#field.clone())))
        });
        let mut kinds = Vec::with_capacity(self.fields.len());
        let mut bindings = Vec::with_capacity(self.fields.len());
        let mut builds = Vec::with_capacity(self.fields.len());
        for (index, (field, ty)) in self.fields.iter().enumerate() {
            let binding = format_ident!("__arg_{index}", span = field.span());
            let (kind, read) = Self::kind_and_read(ty, &binding)?;
            kinds.push(kind);
            bindings.push(binding);
            builds.push(quote!(#field: #read));
        }
        let from_args = self.all_arg.then(|| {
            quote!(
                fn from_args(values: &[crate::rules::ArgValue]) -> Option<Self> {
                    let [#(#bindings,)*] = values else { return None };
                    Some(Self { #(#builds,)* })
                }
            )
        });
        Ok(quote! {
            impl #name {
                #(#consts)*
            }
            impl crate::rules::traits::event_args::EventArgs for #name {
                fn arg(&self, id: crate::rules::ArgId) -> Option<crate::rules::ArgValue> {
                    match *id {
                        #(#reads,)*
                        _ => None,
                    }
                }
                fn args() -> &'static [crate::rules::ArgId] {
                    &[#(Self::#uppers,)*]
                }
                fn kinds() -> &'static [crate::rules::ArgKind] {
                    &[#(#kinds,)*]
                }
                #from_args
            }
        })
    }

    pub(crate) fn kind_and_read(
        ty: &syn::Type,
        value: &syn::Ident,
    ) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
        let syn::Type::Path(path) = ty else {
            return Err(syn::Error::new_spanned(ty, "unsupported #[arg] type"));
        };
        let Some(last) = path.path.segments.last() else {
            return Err(syn::Error::new_spanned(ty, "unsupported #[arg] type"));
        };
        match last.ident.to_string().as_str() {
            "Entity" => Ok((
                quote!(crate::rules::ArgKind::Entity),
                quote!(#value.entity()?),
            )),
            "CardId" => Ok((
                quote!(crate::rules::ArgKind::Entity),
                quote!(crate::cards::CardId(#value.entity()?)),
            )),
            "PlayerId" => Ok((
                quote!(crate::rules::ArgKind::Player),
                quote!(#value.player()?),
            )),
            "u32" => Ok((
                quote!(crate::rules::ArgKind::Amount),
                quote!(#value.amount()?),
            )),
            "EventId" => Ok((
                quote!(crate::rules::ArgKind::Event),
                quote!(#value.event()?),
            )),
            "Vec" => {
                let syn::PathArguments::AngleBracketed(bracketed) = &last.arguments else {
                    return Err(syn::Error::new_spanned(ty, "unsupported #[arg] type"));
                };
                let [syn::GenericArgument::Type(syn::Type::Path(inner))] =
                    bracketed.args.iter().collect::<Vec<_>>().as_slice()
                else {
                    return Err(syn::Error::new_spanned(ty, "unsupported #[arg] type"));
                };
                let is_entity =
                    inner.path.segments.last().is_some_and(|segment| segment.ident == "Entity");
                if !is_entity {
                    return Err(syn::Error::new_spanned(ty, "unsupported #[arg] type"));
                }
                Ok((
                    quote!(crate::rules::ArgKind::Entities),
                    quote!(#value.entities()?),
                ))
            }
            _ => Err(syn::Error::new_spanned(ty, "unsupported #[arg] type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EventArgsExpansion;
    use quote::quote;

    #[test]
    fn test_usage() {
        let input = quote! {
            struct DealDamage {
                #[arg]
                pub source: Entity,
                #[arg]
                pub amount: u32,
                pub other_field: u8,
            }
        };
        let expanded = EventArgsExpansion::expanded(input).unwrap().to_string();
        assert!(expanded.contains("pub const SOURCE"));
        assert!(expanded.contains("pub const AMOUNT"));
        assert!(!expanded.contains("OTHER_FIELD"));
        assert!(expanded.contains("crate :: rules :: ArgId"));
        assert!(expanded.contains("crate :: rules :: ArgValue :: from"));
        assert!(expanded.contains("Self :: SOURCE"));
        assert!(expanded.contains("fn args"));
        assert!(expanded.contains("fn arg"));
        assert!(expanded.contains("_ => None"));
        assert!(expanded.contains("fn kinds"));
        assert!(expanded.contains("ArgKind :: Entity"));
        assert!(expanded.contains("ArgKind :: Amount"));
        assert!(!expanded.contains("fn from_args"));
        let all_arg = quote! {
            struct Cast {
                #[arg]
                pub spell: Entity,
                #[arg]
                pub caster: PlayerId,
            }
        };
        let expanded_all = EventArgsExpansion::expanded(all_arg).unwrap().to_string();
        assert!(expanded_all.contains("fn kinds"));
        assert!(expanded_all.contains("fn from_args"));
        assert!(expanded_all.contains(". entity ()"));
        assert!(expanded_all.contains(". player ()"));
        let mixed = quote! {
            struct AddPhase {
                #[arg]
                pub player: PlayerId,
                pub phase: Phase,
            }
        };
        let expanded_mixed = EventArgsExpansion::expanded(mixed).unwrap().to_string();
        assert!(expanded_mixed.contains("fn kinds"));
        assert!(!expanded_mixed.contains("fn from_args"));
        let unsupported_arg = quote! {
            struct Odd {
                #[arg]
                pub phase: Phase,
            }
        };
        assert!(EventArgsExpansion::expanded(unsupported_arg).is_err());
    }

    #[test]
    fn test_usage_rejects_tuple_struct() {
        let input = quote! {
            struct Pair(Entity, u32);
        };
        assert!(EventArgsExpansion::expanded(input).is_err());
    }

    #[test]
    fn test_usage_rejects_enum() {
        let input = quote! {
            enum Choice {
                Heads,
                Tails,
            }
        };
        assert!(EventArgsExpansion::expanded(input).is_err());
    }
}
