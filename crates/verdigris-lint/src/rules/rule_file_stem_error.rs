use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuleFileStemError {
    #[error(
        "`{stem}` does not start with a rule id (expected `r<digits>_<digits>...`, e.g. `r117_4_priority`)"
    )]
    MissingRuleId { stem: String },
    #[error(
        "`{stem}` has an invalid rule id part `{part}` (expected `<digits>_<digits><lowercase letters>`, e.g. `117_4` or `608_2b`)"
    )]
    InvalidRuleId { stem: String, part: String },
    #[error("`{stem}` has rule ids but no slug (expected `r<id>_<slug>`, e.g. `r117_4_priority`)")]
    MissingSlug { stem: String },
    #[error(
        "`{stem}` has an invalid slug `{slug}` (expected snake_case not starting with `r<digit>`, e.g. `priority`)"
    )]
    InvalidSlug { stem: String, slug: String },
}
