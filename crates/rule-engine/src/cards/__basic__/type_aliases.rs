use bevy::prelude::{Or, With};

use crate::cards;

pub type InCombat = Or<(
    With<cards::Attacking>,
    With<cards::Blocking>,
    With<cards::Blocked>,
)>;
