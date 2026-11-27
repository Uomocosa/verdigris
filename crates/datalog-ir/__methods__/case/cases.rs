use crate::Case;

pub fn cases() -> Vec<Case> {
    vec![
        Case::transitive_closure(),
        Case::stratified_negation(),
        Case::comparison_literal(),
        Case::constants_in_head_and_body(),
        Case::two_independent_strata(),
        Case::negated_relation_empty(),
        Case::unstratifiable(),
        Case::unsafe_head(),
    ]
}

#[cfg(test)]
mod tests {
    use super::cases;

    #[test]
    fn test_usage() {
        let all = cases();
        assert_eq!(all.len(), 8);
        assert_eq!(all[1].name, "stratified_negation");
        assert!(all.iter().any(|case| case.name == "unsafe_head"));
    }
}
