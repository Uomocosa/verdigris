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
    use crate::CREATURE;
    use crate::keywords;
    use crate::rules;
    use crate::statics;

    use super::as_long_as;

    #[test]
    fn test_usage() {
        let ability = as_long_as(
            statics::all(CREATURE).gain(keywords::vigilance()),
            rules::life().at_least(30).of(rules::YOU),
        );
        assert_eq!(
            ability.as_long_as,
            Some(rules::life().at_least(30).of(rules::YOU))
        );
        assert_eq!(ability.filter, CREATURE);
    }
}
