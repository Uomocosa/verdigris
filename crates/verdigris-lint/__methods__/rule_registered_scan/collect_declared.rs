use crate::rules::rule_registered_scan::RuleRegisteredScan;

pub fn collect_declared(scan: &mut RuleRegisteredScan, file: &syn::File) {
    for item in &file.items {
        if let syn::Item::Mod(found) = item
            && found.content.is_none()
        {
            scan.declared.insert(found.ident.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::RuleRegisteredScan;
    use super::collect_declared;

    #[test]
    fn test_usage() {
        let mut scan = RuleRegisteredScan {
            declared: BTreeSet::new(),
            registered: BTreeSet::new(),
        };
        let file: syn::File = syn::parse_str(
            "pub mod r117_4_priority;\npub mod helpers;\n#[path = \"__basic__/mod.rs\"] mod basic;",
        )
        .unwrap();
        collect_declared(&mut scan, &file);
        assert!(scan.declared.contains("r117_4_priority"));
        assert!(scan.declared.contains("helpers"));
        assert!(scan.declared.contains("basic"));
    }
}
