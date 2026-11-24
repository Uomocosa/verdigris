use bevy::prelude::Commands;

use crate::Attempt;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyEvent {
    WinGame(rules::WinGame),
    LoseGame(rules::LoseGame),
    DrawFromEmptyLibrary(rules::DrawFromEmptyLibrary),
    Draw(rules::Draw),
    LoseLife(rules::LoseLife),
    Discard(rules::Discard),
    GainLife(rules::GainLife),
    Mill(rules::Mill),
    CreateTokens(rules::CreateTokens),
    Destroy(rules::Destroy),
    Counter(rules::Counter),
    DealDamage(rules::DealDamage),
    EnterBattlefield(rules::EnterBattlefield),
    BeginTheGameOnTheBattlefield(rules::BeginTheGameOnTheBattlefield),
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
            Self::Mill(happened) => {
                commands.write_message(Attempt(happened));
            }
            Self::CreateTokens(happened) => {
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
            Self::EnterBattlefield(happened) => {
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
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, MessageReader};

    use crate::Attempt;
    use crate::rules;
    use crate::testing::*;

    use super::AnyEvent;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1, P2]);
        let player = p1.id;
        game.app
            .world_mut()
            .run_system_once(move |mut commands: Commands| {
                AnyEvent::WinGame(rules::WinGame(player)).announce(&mut commands);
            })
            .unwrap();
        game.app
            .world_mut()
            .run_system_once(move |mut reader: MessageReader<Attempt<rules::WinGame>>| {
                assert!(reader.read().any(|attempt| **attempt == rules::WinGame(player)));
            })
            .unwrap();
    }
}
