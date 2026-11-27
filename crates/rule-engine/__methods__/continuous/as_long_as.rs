use crate::abilities;
use crate::rules;

pub fn as_long_as(
    ability: abilities::Continuous,
    condition: rules::Condition,
) -> abilities::Continuous {
    abilities::Continuous {
        as_long_as: Some(condition),
        ..ability
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::creature;
    use crate::keywords;
    use crate::rules;
    use crate::statics;

    use super::as_long_as;

    #[test]
    fn test_usage() {
        let ability = as_long_as(
            statics::all(creature()).gain(keywords::vigilance()),
            rules::life().of(YOU).is(rules::Value(30).or_more()),
        );
        assert_eq!(
            ability.as_long_as,
            Some(rules::life().of(YOU).is(rules::Value(30).or_more()))
        );
        assert_eq!(ability.filter, creature());
    }
}
