use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::MessageReader;

use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    SecondCombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

#[atomic_delegates]
impl Step {
    pub fn begins(self) -> impl FnMut(MessageReader<turns::StepStarted>) -> bool + Clone {}
}

#[rustfmt::skip]
impl Step {
    #[must_use]
    pub fn begins_a_phase(self) -> bool { self.phase().steps().first() == Some(&self) }
}

impl Step {
    #[must_use]
    pub const fn phase(self) -> turns::Phase {
        match self {
            Self::Untap | Self::Upkeep | Self::Draw => turns::Phase::Beginning,
            Self::PrecombatMain => turns::Phase::PrecombatMain,
            Self::BeginningOfCombat
            | Self::DeclareAttackers
            | Self::DeclareBlockers
            | Self::CombatDamage
            | Self::SecondCombatDamage
            | Self::EndOfCombat => turns::Phase::Combat,
            Self::PostcombatMain => turns::Phase::PostcombatMain,
            Self::End | Self::Cleanup => turns::Phase::Ending,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::turns;

    use super::Step;

    #[test]
    fn test_usage() {
        let phases = [
            (Step::Untap, turns::Phase::Beginning),
            (Step::Upkeep, turns::Phase::Beginning),
            (Step::Draw, turns::Phase::Beginning),
            (Step::PrecombatMain, turns::Phase::PrecombatMain),
            (Step::BeginningOfCombat, turns::Phase::Combat),
            (Step::DeclareAttackers, turns::Phase::Combat),
            (Step::DeclareBlockers, turns::Phase::Combat),
            (Step::CombatDamage, turns::Phase::Combat),
            (Step::SecondCombatDamage, turns::Phase::Combat),
            (Step::EndOfCombat, turns::Phase::Combat),
            (Step::PostcombatMain, turns::Phase::PostcombatMain),
            (Step::End, turns::Phase::Ending),
            (Step::Cleanup, turns::Phase::Ending),
        ];

        for (step, phase) in phases {
            assert_eq!(step.phase(), phase);
        }
        assert!(Step::Untap.begins_a_phase());
        assert!(Step::PrecombatMain.begins_a_phase());
        assert!(Step::BeginningOfCombat.begins_a_phase());
        assert!(Step::PostcombatMain.begins_a_phase());
        assert!(Step::End.begins_a_phase());
        assert!(!Step::Upkeep.begins_a_phase());
        assert!(!Step::Draw.begins_a_phase());
        assert!(!Step::CombatDamage.begins_a_phase());
        assert!(!Step::SecondCombatDamage.begins_a_phase());
        assert!(!Step::EndOfCombat.begins_a_phase());
        assert!(!Step::Cleanup.begins_a_phase());
    }
}
