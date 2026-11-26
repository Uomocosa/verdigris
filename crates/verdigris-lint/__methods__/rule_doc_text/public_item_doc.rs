use lele_lint::ItemMarker;

pub fn public_item_doc(file: &syn::File) -> Option<String> {
    for item in &file.items {
        let (visibility, attributes) = match item {
            syn::Item::Const(found) => (&found.vis, &found.attrs),
            syn::Item::Enum(found) => (&found.vis, &found.attrs),
            syn::Item::Fn(found) => (&found.vis, &found.attrs),
            syn::Item::Static(found) => (&found.vis, &found.attrs),
            syn::Item::Struct(found) => (&found.vis, &found.attrs),
            syn::Item::Trait(found) => (&found.vis, &found.attrs),
            syn::Item::Type(found) => (&found.vis, &found.attrs),
            syn::Item::Union(found) => (&found.vis, &found.attrs),
            _ => continue,
        };
        if !matches!(visibility, syn::Visibility::Public(_)) {
            continue;
        }
        if attributes.iter().any(|attribute| ItemMarker(attribute).is_cfg_test()) {
            continue;
        }
        let mut doc = String::new();
        for attribute in attributes {
            if !attribute.path().is_ident("doc") {
                continue;
            }
            if let syn::Meta::NameValue(value) = &attribute.meta
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
    use super::public_item_doc;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("/// 702.9b A creature with flying.\npub struct Flying;\n").unwrap();
        assert_eq!(
            public_item_doc(&file),
            Some(String::from(" 702.9b A creature with flying.\n"))
        );
    }

    #[test]
    fn private_items_have_no_public_doc() {
        let file: syn::File = syn::parse_str("/// A helper.\nfn helper() {}\n").unwrap();
        assert_eq!(public_item_doc(&file), None);
    }

    #[test]
    fn test_modules_are_ignored() {
        let file: syn::File =
            syn::parse_str("#[cfg(test)] mod tests {\n/// A test.\n#[test] fn some_case() {}\n}\n")
                .unwrap();
        assert_eq!(public_item_doc(&file), None);
    }

    #[test]
    fn undocumented_public_item_has_an_empty_doc() {
        let file: syn::File = syn::parse_str("pub struct Flying;\n").unwrap();
        assert_eq!(public_item_doc(&file), Some(String::new()));
    }
}
