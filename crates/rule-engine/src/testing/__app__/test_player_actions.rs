use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::actions;
use crate::testing;
use crate::turns;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPlayerActions {
    pub(crate) player: testing::TestPlayer,
    pub(crate) steps: Vec<testing::PlayStep>,
}

#[atomic_delegates]
impl TestPlayerActions {
    #[must_use]
    pub const fn of(player: testing::TestPlayer) -> Self {}

    #[must_use]
    pub fn from_cast(player: testing::TestPlayer, spell: Entity) -> Self {}
}

impl TestPlayerActions {
    #[must_use]
    pub fn targeting(self, targets: impl Into<testing::Targets>) -> Self {
        let step = testing::PlayStep::Act(
            *self.player,
            actions::Action::ChooseTargets(targets.into().into_entities()),
        );
        self.with_step(step)
    }

    #[must_use]
    pub fn pass_priority(self) -> Self {
        let step = testing::PlayStep::Act(*self.player, actions::Action::PassPriority);
        self.with_step(step)
    }

    #[must_use]
    pub fn pass_to(self, step: turns::Step) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::StepStarts(step),
        })
    }

    #[must_use]
    pub fn end_current_step(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::NextStep,
        })
    }

    #[must_use]
    pub fn end_phase(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::NextPhase,
        })
    }

    #[must_use]
    pub fn end_turn(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::OwnNextTurn,
        })
    }
}

impl TestPlayerActions {
    pub(crate) fn with_step(mut self, step: testing::PlayStep) -> Self {
        self.steps.push(step);
        self
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;
    use crate::turns;

    use super::TestPlayerActions;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let target = world.spawn_empty().id();

        let actions = testing::TestPlayer::from_entity_and_seat(caster, testing::PlayerSeat(1))
            .cast(spell)
            .targeting(target)
            .pass_to(turns::Step::PostcombatMain);

        assert_eq!(
            actions.steps,
            vec![
                testing::PlayStep::Act(caster, actions::Action::Cast(spell)),
                testing::PlayStep::Act(caster, actions::Action::ChooseTargets(vec![target])),
                testing::PlayStep::AutoPass {
                    player: caster,
                    until: testing::AutoPassUntil::StepStarts(turns::Step::PostcombatMain),
                },
            ]
        );

        assert_eq!(
            TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                caster,
                testing::PlayerSeat(1)
            ))
            .steps,
            Vec::new()
        );
    }
}
