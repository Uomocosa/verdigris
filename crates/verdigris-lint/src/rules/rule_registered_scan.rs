use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;
use lele_lint::ItemMarker;

pub struct RuleRegisteredScan {
    pub declared: BTreeSet<String>,
    pub registered: BTreeSet<String>,
}

impl<'ast> syn::visit::Visit<'ast> for RuleRegisteredScan {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        let mut tails = path.segments.iter().rev();
        if let (Some(last), Some(previous)) = (tails.next(), tails.next())
            && last.ident == previous.ident
        {
            self.registered.insert(last.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}

#[atomic_delegates]
impl RuleRegisteredScan {
    pub(crate) fn collect_declared(&mut self, file: &syn::File) {}
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use syn::visit::Visit;

    use super::RuleRegisteredScan;

    #[test]
    fn test_usage() {
        let mut scan = RuleRegisteredScan {
            declared: BTreeSet::new(),
            registered: BTreeSet::new(),
        };
        let file: syn::File = syn::parse_str(
            "pub fn add(app: &mut App) { app.add_systems(rules::r117_4_priority::r117_4_priority); }",
        )
        .unwrap();
        scan.visit_file(&file);
        assert!(scan.registered.contains("r117_4_priority"));
        assert!(!scan.registered.contains("r608_2b_illegal_targets"));
    }

    #[test]
    fn single_segment_paths_are_ignored() {
        let mut scan = RuleRegisteredScan {
            declared: BTreeSet::new(),
            registered: BTreeSet::new(),
        };
        let file: syn::File =
            syn::parse_str("pub fn add(app: &mut App) { app.add_systems(Update); }").unwrap();
        scan.visit_file(&file);
        assert!(scan.registered.is_empty());
    }

    #[test]
    fn cfg_test_module_paths_are_ignored() {
        let mut scan = RuleRegisteredScan {
            declared: BTreeSet::new(),
            registered: BTreeSet::new(),
        };
        let file: syn::File = syn::parse_str(
            "#[cfg(test)] mod tests { pub fn probe() { let _ = rules::r117_4_priority::r117_4_priority; } }",
        )
        .unwrap();
        scan.visit_file(&file);
        assert!(scan.registered.is_empty());
    }
}
