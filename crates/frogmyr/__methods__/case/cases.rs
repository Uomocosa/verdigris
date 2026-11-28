use crate::Case;

pub fn cases() -> Vec<Case> {
    vec![
        Case::transitive_closure(),
        Case::stratified_negation(),
        Case::comparison_literal(),
        Case::constants_in_head_and_body(),
        Case::count_grouped_by_head(),
        Case::count_unstratifiable(),
        Case::rules_macro_builds_ir(),
        Case::two_independent_strata(),
        Case::negated_relation_empty(),
        Case::unstratifiable(),
        Case::unsafe_head(),
        Case::entity_traversal(),
    ]
}

#[cfg(test)]
mod tests {
    use super::cases;

    #[test]
    fn test_usage() {
        let all = cases();
        assert_eq!(all.len(), 12);
        assert_eq!(all[1].name, "stratified_negation");
        assert!(all.iter().any(|case| case.name == "unsafe_head"));
        assert!(all.iter().any(|case| case.name == "count_grouped_by_head"));
        assert!(all.iter().any(|case| case.name == "count_unstratifiable"));
    }
}
