use bevy::prelude::{Or, With, Without};

use crate::players;

pub type StillInTheGame = (With<players::Player>, Without<players::LeftTheGame>);

pub type LostOrDrawn = Or<(With<players::Lost>, With<players::GameIsADraw>)>;
