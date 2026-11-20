use bevy::prelude::Entity;

use crate::actions;
use crate::testing;
use crate::turns;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayStep {
    Act(Entity, actions::Action),
    Activate(Entity, testing::TestAbility),
    AutoPass {
        player: Entity,
        until: AutoPassUntil,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoPassUntil {
    StepStarts(turns::Step),
    NextStep,
    NextPhase,
    OwnNextTurn,
}
