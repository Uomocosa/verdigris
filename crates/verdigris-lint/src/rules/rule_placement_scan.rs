use std::path::PathBuf;

use lele_lint::ItemMarker;
use lele_lint::RuleViolation;

use crate::rules;

pub struct RulePlacementScan {
    pub violations: Vec<RuleViolation>,
    pub file: PathBuf,
}

impl<'ast> syn::visit::Visit<'ast> for RulePlacementScan {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let is_test = item
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test() || ItemMarker(attr).is_test());
        if !is_test {
            self.check_name(&item.sig.ident.to_string());
        }
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let is_test = item
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test() || ItemMarker(attr).is_test());
        if !is_test {
            self.check_name(&item.sig.ident.to_string());
        }
        syn::visit::visit_impl_item_fn(self, item);
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        let is_test = item
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test() || ItemMarker(attr).is_test());
        if !is_test {
            self.check_name(&item.sig.ident.to_string());
        }
        syn::visit::visit_trait_item_fn(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_trait(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }
}

impl RulePlacementScan {
    pub(crate) fn check_name(&mut self, name: &str) {
        if rules::rule_file_stem::RuleFileStem::try_from(name).is_ok() {
            self.violations.push(RuleViolation::from_file(
                self.file.clone(),
                rules::rule_placement::RulePlacement::CODE,
                format!("rule system `{name}` must live in __rules__/{name}.rs"),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use syn::visit::Visit;

    use super::RulePlacementScan;

    #[test]
    fn test_usage() {
        let mut scan = RulePlacementScan {
            violations: Vec::new(),
            file: PathBuf::from("/crate/src/rules_plugin.rs"),
        };
        let file: syn::File =
            syn::parse_str("pub fn r117_4_priority() {} impl RulesPlugin { pub fn add() {} }")
                .unwrap();
        scan.visit_file(&file);
        assert_eq!(scan.violations.len(), 1);
        assert!(scan.violations[0].message.contains("r117_4_priority"));
    }

    #[test]
    fn cfg_test_module_fns_are_ignored() {
        let mut scan = RulePlacementScan {
            violations: Vec::new(),
            file: PathBuf::from("/crate/src/rules_plugin.rs"),
        };
        let file: syn::File = syn::parse_str(
            "#[cfg(test)] mod tests { #[test] fn r117_4_passing_resolves_the_spell() {} }",
        )
        .unwrap();
        scan.visit_file(&file);
        assert!(scan.violations.is_empty());
    }

    #[test]
    fn test_attribute_fns_are_ignored() {
        let mut scan = RulePlacementScan {
            violations: Vec::new(),
            file: PathBuf::from("/crate/src/rules_plugin.rs"),
        };
        let file: syn::File =
            syn::parse_str("#[test] fn r117_4_passing_resolves_the_spell() {}").unwrap();
        scan.visit_file(&file);
        assert!(scan.violations.is_empty());
    }
}
