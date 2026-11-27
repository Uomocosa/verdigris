use crate::events;
use bevy::prelude::Commands;

use crate::Attempt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyEvent {
    WinGame(events::WinGame),
    LoseGame(events::LoseGame),
    DrawFromEmptyLibrary(events::DrawFromEmptyLibrary),
    Draw(events::Draw),
    LoseLife(events::LoseLife),
    Discard(events::Discard),
    GainLife(events::GainLife),
    PutCounters(events::PutCounters),
    Mill(events::Mill),
    Exile(events::Exile),
    Create(events::Create),
    Destroy(events::Destroy),
    Counter(events::Counter),
    DealDamage(events::DealDamage),
    EnterTheBattlefield(events::EnterTheBattlefield),
    BeginTheGameOnTheBattlefield(events::BeginTheGameOnTheBattlefield),
}

impl AnyEvent {
    pub fn announce(self, commands: &mut Commands) {
        match self {
            Self::WinGame(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::LoseGame(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::DrawFromEmptyLibrary(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Draw(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::LoseLife(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Discard(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::GainLife(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::PutCounters(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Mill(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Exile(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Create(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Destroy(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::Counter(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::DealDamage(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::EnterTheBattlefield(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::BeginTheGameOnTheBattlefield(happened) => {
                commands.write_message(Attempt(happened));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, MessageReader};

    use crate::Attempt;
    use crate::testing::*;

    use super::AnyEvent;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1, P2]);
        let player = p1.id;
        game.app
            .world_mut()
            .run_system_once(move |mut commands: Commands| {
                AnyEvent::WinGame(events::WinGame { player }).announce(&mut commands);
            })
            .unwrap();
        game.app
            .world_mut()
            .run_system_once(move |mut reader: MessageReader<Attempt<events::WinGame>>| {
                assert!(reader.read().any(|attempt| **attempt == events::WinGame { player }));
            })
            .unwrap();
    }
}
