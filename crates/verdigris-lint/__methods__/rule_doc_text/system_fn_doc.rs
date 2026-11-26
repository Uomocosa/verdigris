use lele_lint::ItemMarker;

pub fn system_fn_doc(file: &syn::File, stem: &str) -> Option<String> {
    for item in &file.items {
        let syn::Item::Fn(system) = item else {
            continue;
        };
        if system.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            continue;
        }
        if matches!(system.vis, syn::Visibility::Inherited) {
            continue;
        }
        if system.sig.ident != stem {
            continue;
        }
        let mut doc = String::new();
        for attr in &system.attrs {
            if !attr.path().is_ident("doc") {
                continue;
            }
            if let syn::Meta::NameValue(value) = &attr.meta
                && let syn::Expr::Lit(literal) = &value.value
                && let syn::Lit::Str(text) = &literal.lit
            {
                doc.push_str(text.value().as_str());
                doc.push('\n');
            }
        }
        return Some(doc);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::system_fn_doc;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("/// 117.4. If all players pass.\npub fn r117_4_priority() {}").unwrap();
        assert_eq!(
            system_fn_doc(&file, "r117_4_priority"),
            Some(String::from(" 117.4. If all players pass.\n"))
        );
    }

    #[test]
    fn private_fns_have_no_system_doc() {
        let file: syn::File =
            syn::parse_str("/// 117.4. If all players pass.\nfn r117_4_priority() {}").unwrap();
        assert_eq!(system_fn_doc(&file, "r117_4_priority"), None);
    }

    #[test]
    fn other_named_fns_are_ignored() {
        let file: syn::File =
            syn::parse_str("/// 117.4. If all players pass.\npub fn other() {}").unwrap();
        assert_eq!(system_fn_doc(&file, "r117_4_priority"), None);
    }

    #[test]
    fn undocumented_system_fn_has_an_empty_doc() {
        let file: syn::File = syn::parse_str("pub fn r117_4_priority() {}").unwrap();
        assert_eq!(system_fn_doc(&file, "r117_4_priority"), Some(String::new()));
    }
}
