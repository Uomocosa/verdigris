use atomic_delegate_macros::atomic_delegates;

use crate::effects;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continuous {
    pub filter: effects::Filter,
    pub modification: effects::Modification,
    pub as_long_as: Option<rules::Condition>,
}

#[atomic_delegates]
impl Continuous {
    #[must_use]
    pub fn as_long_as(self, condition: rules::Condition) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::abilities;
    use crate::creature;
    use crate::keywords;
    use crate::rules;
    use crate::statics;

    #[test]
    fn test_usage() {
        let ability = statics::all(creature())
            .gain(keywords::vigilance())
            .as_long_as(rules::life().of(YOU).is(rules::Value(30).or_more()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.as_long_as,
            Some(rules::life().of(YOU).is(rules::Value(30).or_more()))
        );
        assert_eq!(
            statics::all(creature()).gain(keywords::vigilance()),
            abilities::Continuous {
                filter: ability.filter,
                modification: ability.modification,
                as_long_as: None,
            }
        );
    }
}
