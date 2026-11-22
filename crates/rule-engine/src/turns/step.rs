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
    }
}
