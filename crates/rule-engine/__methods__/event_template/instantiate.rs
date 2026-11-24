use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn instantiate<E: Event>(template: &rules::EventTemplate, event: &E) -> rules::AnyEvent {
    let holder = |who: &rules::Role| match who {
        rules::Role::Subject => players::PlayerId(event.subject()),
        rules::Role::Other => players::PlayerId(event.other().unwrap_or_else(|| *event.player())),
        rules::Role::Player => event.player(),
    };
    let amount = |amount: &rules::Amount| match amount {
        rules::Amount::Fixed(fixed) => *fixed,
        rules::Amount::FromEvent(field) => event.field(*field).unwrap_or(0),
    };
    match template {
        rules::EventTemplate::WinGame { who } => {
            rules::AnyEvent::WinGame(rules::WinGame(holder(who)))
        }
        rules::EventTemplate::LoseGame { who } => {
            rules::AnyEvent::LoseGame(rules::LoseGame(holder(who)))
        }
        rules::EventTemplate::Draw {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Draw(rules::Draw {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::LoseLife {
            who,
            amount: template_amount,
        } => rules::AnyEvent::LoseLife(rules::LoseLife {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::Discard {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Discard(rules::Discard {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::GainLife {
            who,
            amount: template_amount,
        } => rules::AnyEvent::GainLife(rules::GainLife {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::Mill {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Mill(rules::Mill {
            player: holder(who),
            amount: amount(template_amount),
        }),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;

    use super::instantiate;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let happened = rules::DrawFromEmptyLibrary(players::PlayerId(player));
        assert_eq!(
            instantiate(
                &rules::EventTemplate::WinGame {
                    who: rules::Role::Player,
                },
                &happened
            ),
            rules::AnyEvent::WinGame(rules::WinGame(players::PlayerId(player)))
        );
    }
}
