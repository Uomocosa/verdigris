use crate::rules;
use crate::rules::traits::event::Event;

pub fn bind<E: Event>(condition: &rules::Condition, event: &E) -> rules::Condition {
    match condition {
        rules::Condition::EventField(field, comparison, threshold) => {
            let holds = event.field(*field).is_some_and(|value| match comparison {
                rules::Comparison::AtLeast => value >= *threshold,
                rules::Comparison::AtMost => value <= *threshold,
                rules::Comparison::EqualTo => value == *threshold,
                rules::Comparison::GreaterThan => value > *threshold,
                rules::Comparison::LessThan => value < *threshold,
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
        let at_least_two = rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 2);
        assert_eq!(bind(&at_least_two, &draw_two), rules::Condition::True);
    }

    #[test]
    fn bind_rewrites_event_field_against_the_event_amount() {
        let draw_two = events::Draw {
            player: players::PlayerId(Entity::PLACEHOLDER),
            amount: 2,
        };
        let at_least_three =
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 3);
        assert_eq!(bind(&at_least_three, &draw_two), !rules::Condition::True);
        let nested = rules::Condition::All(vec![
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 2),
            rules::Condition::True,
        ]);
        assert_eq!(
            bind(&nested, &draw_two),
            rules::Condition::All(vec![rules::Condition::True, rules::Condition::True])
        );
        let missing = rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 1);
        assert_eq!(
            bind(
                &missing,
                &events::WinGame {
                    player: players::PlayerId(Entity::PLACEHOLDER)
                }
            ),
            !rules::Condition::True
        );
    }
}
