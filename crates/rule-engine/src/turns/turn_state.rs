use atomic_delegate_macros::atomic_delegates;
use std::collections::VecDeque;

use bevy::prelude::Resource;

use crate::players;
use crate::turns;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct TurnState {
    pub turn: turns::Turn,
    pub priority: turns::Priority,
    pub(crate) suspended_priority_holder: Option<players::PlayerId>,
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
            suspended_priority_holder: None,
            upcoming_steps: PHASES_IN_ORDER
                .into_iter()
                .flat_map(turns::Phase::steps)
                .copied()
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

    #[must_use]
    pub fn step_after_next(&self) -> Option<turns::Step> { self.upcoming_steps.get(1).copied() }

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

const PHASES_IN_ORDER: [turns::Phase; 5] = [
    turns::Phase::Beginning,
    turns::Phase::PrecombatMain,
    turns::Phase::Combat,
    turns::Phase::PostcombatMain,
    turns::Phase::Ending,
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
        assert_eq!(state.step_after_next(), Some(turns::Step::Draw));
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
