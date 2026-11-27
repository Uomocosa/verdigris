use crate::rules;

pub fn of(
    quantity: rules::PlayerQuantity,
    player: impl Into<rules::ArgRef>,
) -> rules::PlayerQuantityOf {
    rules::PlayerQuantityOf {
        quantity,
        player: player.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::of;

    #[test]
    fn test_usage() {
        assert_eq!(
            of(rules::PlayerQuantity::Life, rules::YOU),
            rules::PlayerQuantityOf {
                quantity: rules::PlayerQuantity::Life,
                player: rules::YOU,
            }
        );
    }
}
