use bevy::prelude::Entity;

use crate::events;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn instantiate<E: Event>(
    template: &rules::EventTemplate,
    event: &E,
    answers: &rules::Rules,
) -> rules::AnyEvent {
    let fallback = E::args()
        .iter()
        .find_map(|id| match event.arg(*id) {
            Some(rules::ArgValue::Player(player)) => Some(player),
            _ => None,
        })
        .unwrap_or(players::PlayerId(Entity::PLACEHOLDER));
    let holder = |who: &rules::ArgRef| match answers.resolve(*who, event) {
        Some(rules::ArgValue::Player(player)) => player,
        Some(rules::ArgValue::Entity(entity)) => players::PlayerId(entity),
        Some(rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_)) | None => fallback,
    };
    let object = |what: &rules::ArgRef| match answers.resolve(*what, event) {
        Some(rules::ArgValue::Entity(entity)) => entity,
        Some(rules::ArgValue::Player(player)) => *player,
        Some(rules::ArgValue::Entities(entities)) => entities.first().copied().unwrap_or(*fallback),
        Some(rules::ArgValue::Amount(_)) | None => *fallback,
    };
    let amount = |amount: &rules::Amount| match amount {
        rules::Amount::Fixed(fixed) => *fixed,
        rules::Amount::FromArg(id) => match event.arg(*id) {
            Some(rules::ArgValue::Amount(found)) => found,
            _ => 0,
        },
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
                card: E::args()
                    .iter()
                    .find_map(|id| match event.arg(*id) {
                        Some(rules::ArgValue::Entity(entity)) => Some(entity),
                        _ => None,
                    })
                    .unwrap_or(Entity::PLACEHOLDER),
                player: fallback,
            })
        }
        rules::EventTemplate::Exile { what } => rules::AnyEvent::Exile(events::Exile {
            object: object(what),
        }),
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::rules;
    use crate::testing::{MockGame, P1, P2};

    use super::instantiate;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let happened = events::DrawFromEmptyLibrary {
            player: players::PlayerId(player),
        };
        let (mut game, _) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    instantiate(
                        &rules::EventTemplate::WinGame {
                            who: events::DrawFromEmptyLibrary::PLAYER.into(),
                        },
                        &happened,
                        &answers,
                    ),
                    rules::AnyEvent::WinGame(events::WinGame {
                        player: players::PlayerId(player)
                    })
                );
                assert_eq!(
                    instantiate(
                        &rules::EventTemplate::Exile {
                            what: events::DrawFromEmptyLibrary::PLAYER.into(),
                        },
                        &happened,
                        &answers,
                    ),
                    rules::AnyEvent::Exile(events::Exile { object: player })
                );
            })
            .unwrap();
    }
}
