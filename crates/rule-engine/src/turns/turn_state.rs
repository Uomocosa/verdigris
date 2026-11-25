use atomic_delegate_macros::atomic_delegates;
use std::collections::VecDeque;

use bevy::prelude::Resource;

use crate::turns;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct TurnState {
    pub turn: turns::Turn,
    pub priority: turns::Priority,
    pub(crate) upcoming_steps: VecDeque<turns::Step>,
}

impl From<turns::Turn> for TurnState {
    fn from(turn: turns::Turn) -> Self {
        let current = match turn.step {
            turns::Step::SecondCombatDamage => turns::Step::CombatDamage,
            current => current,
        };
        Self {
            turn,
            priority: turns::Priority::NO_HOLDER,
            upcoming_steps: STEPS_IN_ORDER
                .into_iter()
                .skip_while(|step| *step != current)
                .skip(1)
                .collect(),
        }
    }
}

#[rustfmt::skip]
impl TurnState {
    #[must_use]
    pub fn next_step(&self) -> Option<turns::Step> { self.upcoming_steps.front().copied() }

    pub fn add_step_after_this_one(&mut self, step: turns::Step) { self.upcoming_steps.push_front(step); }
}

#[atomic_delegates]
impl TurnState {
    pub fn add_phase_after_this_one(&mut self, phase: turns::Phase) {}
}

impl TurnState {
    pub fn begin_step(&mut self, step: turns::Step) {
        self.turn.step = step;
        if let Some(position) = self.upcoming_steps.iter().position(|next| *next == step) {
            self.upcoming_steps.drain(..=position).for_each(drop);
        }
    }
}

const STEPS_IN_ORDER: [turns::Step; 12] = [
    turns::Step::Untap,
    turns::Step::Upkeep,
    turns::Step::Draw,
    turns::Step::PrecombatMain,
    turns::Step::BeginningOfCombat,
    turns::Step::DeclareAttackers,
    turns::Step::DeclareBlockers,
    turns::Step::CombatDamage,
    turns::Step::EndOfCombat,
    turns::Step::PostcombatMain,
    turns::Step::End,
    turns::Step::Cleanup,
];

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::turns;

    use super::TurnState;

    #[test]
    fn test_usage() {
        let mut state = TurnState::from(turns::Turn {
            number: 1,
            active: players::PlayerId(Entity::PLACEHOLDER),
            step: turns::Step::Untap,
        });
        assert_eq!(state.turn.number, 1);
        assert_eq!(state.priority, turns::Priority::NO_HOLDER);
        assert_eq!(state.upcoming_steps.len(), 11);
        assert_eq!(state.next_step(), Some(turns::Step::Upkeep));
        state.add_step_after_this_one(turns::Step::SecondCombatDamage);
        state.add_step_after_this_one(turns::Step::EndOfCombat);
        assert_eq!(state.next_step(), Some(turns::Step::EndOfCombat));
        state.begin_step(turns::Step::CombatDamage);
        assert_eq!(state.turn.step, turns::Step::CombatDamage);
        assert_eq!(state.next_step(), Some(turns::Step::EndOfCombat));
        let unchanged = state.upcoming_steps.clone();
        state.begin_step(turns::Step::SecondCombatDamage);
        assert_eq!(state.upcoming_steps, unchanged);
    }
}
