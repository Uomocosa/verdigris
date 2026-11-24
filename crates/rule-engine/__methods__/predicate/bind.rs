use crate::rules;
use crate::rules::traits::event::Event;

pub fn bind<E: Event>(predicate: &rules::Predicate, event: &E) -> rules::Predicate {
    match predicate {
        rules::Predicate::EventField(field, comparison, threshold) => {
            let holds = event.field(*field).is_some_and(|value| match comparison {
                rules::Comparison::AtLeast => value >= *threshold,
                rules::Comparison::AtMost => value <= *threshold,
                rules::Comparison::EqualTo => value == *threshold,
            });
            if holds {
                rules::Predicate::True
            } else {
                !rules::Predicate::True
            }
        }
        rules::Predicate::Of(role, inner) => {
            rules::Predicate::Of(*role, Box::new(bind(inner, event)))
        }
        rules::Predicate::All(predicates) => {
            rules::Predicate::All(predicates.iter().map(|inner| bind(inner, event)).collect())
        }
        rules::Predicate::Any(predicates) => {
            rules::Predicate::Any(predicates.iter().map(|inner| bind(inner, event)).collect())
        }
        rules::Predicate::Not(inner) => rules::Predicate::Not(Box::new(bind(inner, event))),
        _ => predicate.clone(),
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
        assert_eq!(bind(&at_least_two, &draw_two), rules::Predicate::True);
    }

    #[test]
    fn bind_rewrites_event_field_against_the_event_amount() {
        let draw_two = events::Draw {
            player: players::PlayerId(Entity::PLACEHOLDER),
            amount: 2,
        };
        let at_least_three =
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 3);
        assert_eq!(bind(&at_least_three, &draw_two), !rules::Predicate::True);
        let nested = rules::Predicate::All(vec![
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 2),
            rules::Predicate::True,
        ]);
        assert_eq!(
            bind(&nested, &draw_two),
            rules::Predicate::All(vec![rules::Predicate::True, rules::Predicate::True])
        );
        let missing = rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 1);
        assert_eq!(
            bind(
                &missing,
                &events::WinGame(players::PlayerId(Entity::PLACEHOLDER))
            ),
            !rules::Predicate::True
        );
    }
}
