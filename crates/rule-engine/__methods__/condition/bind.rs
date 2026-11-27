use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn bind<E: EventArgs>(condition: &rules::Condition, event: &E) -> rules::Condition {
    match condition {
        rules::Condition::ArgAmount(id, comparison) | rules::Condition::Count(id, comparison) => {
            let holds = event
                .arg(*id)
                .and_then(|value| match value {
                    rules::ArgValue::Amount(found) => Some(found),
                    rules::ArgValue::Entities(found) => found.len().try_into().ok(),
                    _ => None,
                })
                .is_some_and(|value| {
                    i32::try_from(value).is_ok_and(|narrow| comparison.holds(narrow))
                });
            if holds {
                rules::Condition::True
            } else {
                !rules::Condition::True
            }
        }
        rules::Condition::Of(role, inner) => {
            rules::Condition::Of(*role, Box::new(bind(inner, event)))
        }
        rules::Condition::Each(set, inner) => {
            rules::Condition::Each(*set, Box::new(bind(inner, event)))
        }
        rules::Condition::AnyPlayer(set, inner) => {
            rules::Condition::AnyPlayer(*set, Box::new(bind(inner, event)))
        }
        rules::Condition::All(conditions) => {
            rules::Condition::All(conditions.iter().map(|inner| bind(inner, event)).collect())
        }
        rules::Condition::Any(conditions) => {
            rules::Condition::Any(conditions.iter().map(|inner| bind(inner, event)).collect())
        }
        rules::Condition::Not(inner) => rules::Condition::Not(Box::new(bind(inner, event))),
        _ => condition.clone(),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::rules;

    use super::bind;

    #[test]
    fn test_usage() {
        let draw_two = events::Draw {
            player: players::PlayerId(Entity::PLACEHOLDER),
            amount: 2,
            cause: rules::EventId("608.2/resolve"),
        };
        let at_least_two = events::Draw::AMOUNT.is(rules::Value(2).or_more());
        assert_eq!(bind(&at_least_two, &draw_two), rules::Condition::True);
    }

    #[test]
    fn bind_rewrites_event_field_against_the_event_amount() {
        let draw_two = events::Draw {
            player: players::PlayerId(Entity::PLACEHOLDER),
            amount: 2,
            cause: rules::EventId("608.2/resolve"),
        };
        let at_least_three = events::Draw::AMOUNT.is(rules::Value(3).or_more());
        assert_eq!(bind(&at_least_three, &draw_two), !rules::Condition::True);
        let nested = rules::Condition::All(vec![
            events::Draw::AMOUNT.is(rules::Value(2).or_more()),
            rules::Condition::True,
        ]);
        assert_eq!(
            bind(&nested, &draw_two),
            rules::Condition::All(vec![rules::Condition::True, rules::Condition::True])
        );
        let missing = events::Draw::AMOUNT.is(rules::Value(1).or_more());
        assert_eq!(
            bind(
                &missing,
                &events::WinGame {
                    player: players::PlayerId(Entity::PLACEHOLDER)
                },
            ),
            !rules::Condition::True
        );
    }
}
