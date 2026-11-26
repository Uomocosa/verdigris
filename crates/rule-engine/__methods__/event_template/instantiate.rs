use crate::events;
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
        rules::EventTemplate::WinGame { who } => rules::AnyEvent::WinGame(events::WinGame {
            player: holder(who),
        }),
        rules::EventTemplate::LoseGame { who } => rules::AnyEvent::LoseGame(events::LoseGame {
            player: holder(who),
        }),
        rules::EventTemplate::Draw {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Draw(events::Draw {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::LoseLife {
            who,
            amount: template_amount,
        } => rules::AnyEvent::LoseLife(events::LoseLife {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::Discard {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Discard(events::Discard {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::GainLife {
            who,
            amount: template_amount,
        } => rules::AnyEvent::GainLife(events::GainLife {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::Mill {
            who,
            amount: template_amount,
        } => rules::AnyEvent::Mill(events::Mill {
            player: holder(who),
            amount: amount(template_amount),
        }),
        rules::EventTemplate::BeginTheGameOnTheBattlefield => {
            rules::AnyEvent::BeginTheGameOnTheBattlefield(events::BeginTheGameOnTheBattlefield {
                card: event.subject(),
                player: event.player(),
            })
        }
        rules::EventTemplate::Exile { what } => rules::AnyEvent::Exile(events::Exile {
            object: *holder(what),
            owner: event.player(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::rules;

    use super::instantiate;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let happened = events::DrawFromEmptyLibrary {
            player: players::PlayerId(player),
        };
        assert_eq!(
            instantiate(
                &rules::EventTemplate::WinGame {
                    who: rules::Role::Player,
                },
                &happened
            ),
            rules::AnyEvent::WinGame(events::WinGame {
                player: players::PlayerId(player)
            })
        );
        assert_eq!(
            instantiate(
                &rules::EventTemplate::Exile {
                    what: rules::Role::Subject,
                },
                &happened
            ),
            rules::AnyEvent::Exile(events::Exile {
                object: player,
                owner: players::PlayerId(player),
            })
        );
    }
}
