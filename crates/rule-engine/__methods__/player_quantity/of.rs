use crate::rules;

pub fn of(
    quantity: rules::PlayerQuantity,
    player: impl Into<rules::Subject>,
) -> rules::PlayerQuantityOf {
    rules::PlayerQuantityOf {
        quantity,
        player: player.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::rules;

    use super::of;

    #[test]
    fn test_usage() {
        assert_eq!(
            of(rules::PlayerQuantity::Life, YOU),
            rules::PlayerQuantityOf {
                quantity: rules::PlayerQuantity::Life,
                player: YOU.into(),
            }
        );
    }
}
