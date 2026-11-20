use atomic_delegate_macros::atomic_delegates;

use crate::rules;

pub struct RuleFileStem {
    pub ids: Vec<String>,
    pub slug: String,
}

#[atomic_delegates]
impl TryFrom<&str> for RuleFileStem {
    type Error = rules::rule_file_stem_error::RuleFileStemError;
    fn try_from(stem: &str) -> Result<Self, Self::Error> {}
}

#[cfg(test)]
mod tests {
    use super::RuleFileStem;

    #[test]
    fn test_usage() {
        let parsed = RuleFileStem::try_from("r305_6_r605_3_basic_land_mana_ability").unwrap();
        assert_eq!(parsed.ids, ["305_6", "605_3"]);
        assert_eq!(parsed.slug, "basic_land_mana_ability");
    }
}
