use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn bind<E: EventArgs>(condition: &rules::Condition, event: &E) -> rules::Condition {
    match condition {
        rules::Condition::ArgAmount(id, comparison, threshold)
        | rules::Condition::Count(id, comparison, threshold) => {
            let holds = event
                .arg(*id)
                .and_then(|value| match value {
                    rules::ArgValue::Amount(found) => Some(found),
                    rules::ArgValue::Entities(found) => found.len().try_into().ok(),
                    _ => None,
                })
                .is_some_and(|value| comparison.holds(&value, threshold));
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
        };
        let at_least_two = events::Draw::AMOUNT.at_least(2);
        assert_eq!(bind(&at_least_two, &draw_two), rules::Condition::True);
    }

    #[test]
    fn bind_rewrites_event_field_against_the_event_amount() {
        let draw_two = events::Draw {
            player: players::PlayerId(Entity::PLACEHOLDER),
            amount: 2,
        };
        let at_least_three = events::Draw::AMOUNT.at_least(3);
        assert_eq!(bind(&at_least_three, &draw_two), !rules::Condition::True);
        let nested = rules::Condition::All(vec![
            events::Draw::AMOUNT.at_least(2),
            rules::Condition::True,
        ]);
        assert_eq!(
            bind(&nested, &draw_two),
            rules::Condition::All(vec![rules::Condition::True, rules::Condition::True])
        );
        let missing = events::Draw::AMOUNT.at_least(1);
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
