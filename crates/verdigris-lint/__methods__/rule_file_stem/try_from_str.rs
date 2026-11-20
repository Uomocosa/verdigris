use crate::rules;

pub fn try_from_str(
    stem: &str,
) -> Result<rules::rule_file_stem::RuleFileStem, rules::rule_file_stem_error::RuleFileStemError> {
    let is_id_head = |part: &str| {
        part.strip_prefix('r').is_some_and(|digits| {
            !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
        })
    };
    let is_id_tail = |part: &str| {
        let digits = part.bytes().take_while(u8::is_ascii_digit).count();
        digits > 0 && part.bytes().skip(digits).all(|byte| byte.is_ascii_lowercase())
    };
    let is_slug_part = |part: &str| {
        part.bytes().next().is_some_and(|first| first.is_ascii_lowercase())
            && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    };
    let mut parts = stem.split('_');
    let head = parts.next().unwrap_or_default();
    let tail = parts.next().unwrap_or_default();
    if !is_id_head(head) {
        return Err(
            rules::rule_file_stem_error::RuleFileStemError::MissingRuleId {
                stem: stem.to_string(),
            },
        );
    }
    if !is_id_tail(tail) {
        return Err(
            rules::rule_file_stem_error::RuleFileStemError::InvalidRuleId {
                stem: stem.to_string(),
                part: tail.to_string(),
            },
        );
    }
    let mut ids = vec![format!("{}_{tail}", head.strip_prefix('r').unwrap_or(head))];
    let collected: Vec<&str> = parts.collect();
    let mut rest: &[&str] = collected.as_slice();
    loop {
        match rest {
            [head, tail, after @ ..] if is_id_head(head) && is_id_tail(tail) => {
                ids.push(format!("{}_{tail}", head.strip_prefix('r').unwrap_or(head)));
                rest = after;
            }
            _ => break,
        }
    }
    if rest.is_empty() {
        return Err(
            rules::rule_file_stem_error::RuleFileStemError::MissingSlug {
                stem: stem.to_string(),
            },
        );
    }
    let starts_with_rule_id = rest.first().is_some_and(|part| {
        part.strip_prefix('r')
            .is_some_and(|after| after.bytes().next().is_some_and(|byte| byte.is_ascii_digit()))
    });
    let slug = rest.join("_");
    if starts_with_rule_id || !rest.iter().all(|part| is_slug_part(part)) {
        return Err(
            rules::rule_file_stem_error::RuleFileStemError::InvalidSlug {
                stem: stem.to_string(),
                slug,
            },
        );
    }
    Ok(rules::rule_file_stem::RuleFileStem { ids, slug })
}

#[cfg(test)]
mod tests {
    use super::try_from_str;
    use crate::rules;

    #[test]
    fn test_usage() {
        let parsed = try_from_str("r117_4_priority").unwrap();
        assert_eq!(parsed.ids, ["117_4"]);
        assert_eq!(parsed.slug, "priority");
    }

    #[test]
    fn valid_rule_file_names_parse() {
        let parsed = try_from_str("r304_1_r304_2_name").unwrap();
        assert_eq!(parsed.ids, ["304_1", "304_2"]);
        assert_eq!(parsed.slug, "name");
        let parsed = try_from_str("r305_6a_r305_6b_name").unwrap();
        assert_eq!(parsed.ids, ["305_6a", "305_6b"]);
        assert_eq!(parsed.slug, "name");
        let parsed = try_from_str("r305_6_r605_3_basic_land_mana_ability").unwrap();
        assert_eq!(parsed.ids, ["305_6", "605_3"]);
        assert_eq!(parsed.slug, "basic_land_mana_ability");
    }

    #[test]
    fn invalid_rule_file_names_fail() {
        for stem in [
            "helpers",
            "r117_priority",
            "r117_4",
            "r117_4_R2_x",
            "rule_117_4_x",
            "r117_4_r2",
        ] {
            assert!(try_from_str(stem).is_err(), "{stem}");
        }
    }

    #[test]
    fn failures_name_the_wrong_part() {
        assert!(matches!(
            try_from_str("helpers"),
            Err(rules::rule_file_stem_error::RuleFileStemError::MissingRuleId { .. })
        ));
        assert!(matches!(
            try_from_str("r117_priority"),
            Err(rules::rule_file_stem_error::RuleFileStemError::InvalidRuleId { .. })
        ));
        assert!(matches!(
            try_from_str("r117_4"),
            Err(rules::rule_file_stem_error::RuleFileStemError::MissingSlug { .. })
        ));
        assert!(matches!(
            try_from_str("r117_4_R2_x"),
            Err(rules::rule_file_stem_error::RuleFileStemError::InvalidSlug { .. })
        ));
    }
}
