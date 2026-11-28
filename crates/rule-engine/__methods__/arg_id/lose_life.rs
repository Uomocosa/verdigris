use crate::rules;

pub fn lose_life(arg_id: rules::ArgId, amount: impl Into<rules::Amount>) -> rules::EventTemplate {
    rules::EventTemplate::LoseLife {
        who: arg_id.into(),
        amount: amount.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::lose_life;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("player");
        assert_eq!(
            lose_life(id, 2u32),
            rules::EventTemplate::LoseLife {
                who: id.into(),
                amount: rules::Amount::Fixed(2),
            }
        );
        assert_eq!(
            lose_life(id, id),
            rules::EventTemplate::LoseLife {
                who: id.into(),
                amount: rules::Amount::FromArg(id),
            }
        );
    }
}
