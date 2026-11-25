use std::collections::VecDeque;

use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::turns;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct UpcomingSteps(VecDeque<turns::Step>);

impl From<turns::Step> for UpcomingSteps {
    fn from(current: turns::Step) -> Self {
        let current = match current {
            turns::Step::SecondCombatDamage => turns::Step::CombatDamage,
            current => current,
        };
        Self(STEPS_IN_ORDER.into_iter().skip_while(|step| *step != current).skip(1).collect())
    }
}

#[rustfmt::skip]
impl UpcomingSteps {
    pub const EMPTY: Self = Self(VecDeque::new());
}

#[rustfmt::skip]
impl UpcomingSteps {
    #[must_use]
    pub fn next_step(&self) -> Option<turns::Step> { self.front().copied() }

    pub fn add_directly_after_this_step(&mut self, step: turns::Step) { self.push_front(step); }
}

impl UpcomingSteps {
    pub fn proceed_to(&mut self, step: turns::Step) {
        if let Some(position) = self.iter().position(|next| *next == step) {
            self.drain(..=position).for_each(drop);
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
    use super::UpcomingSteps;
    use crate::turns;

    #[test]
    fn test_usage() {
        let mut upcoming = UpcomingSteps::from(turns::Step::Untap);
        assert_eq!(upcoming.len(), 11);
        assert_eq!(upcoming.next_step(), Some(turns::Step::Upkeep));
        upcoming.add_directly_after_this_step(turns::Step::SecondCombatDamage);
        upcoming.add_directly_after_this_step(turns::Step::EndOfCombat);
        assert_eq!(upcoming.next_step(), Some(turns::Step::EndOfCombat));
        upcoming.proceed_to(turns::Step::CombatDamage);
        assert_eq!(upcoming.next_step(), Some(turns::Step::EndOfCombat));
        let unchanged = upcoming.clone();
        upcoming.proceed_to(turns::Step::SecondCombatDamage);
        assert_eq!(upcoming, unchanged);
    }
}
