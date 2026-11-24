use bevy::prelude::{With, Without};

use crate::players;

pub type StillInTheGame = (
    With<players::Player>,
    Without<players::Lost>,
    Without<players::Drawn>,
);
