use crate::rules;

pub fn of(
    characteristic: rules::Characteristic,
    who: impl Into<rules::Subject>,
) -> rules::CharacteristicOf {
    rules::CharacteristicOf {
        characteristic,
        subject: who.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::THIS;
    use crate::rules;

    use super::of;

    #[test]
    fn test_usage() {
        assert_eq!(
            of(rules::Characteristic::Power, THIS),
            rules::CharacteristicOf {
                characteristic: rules::Characteristic::Power,
                subject: THIS.into(),
            }
        );
    }
}
