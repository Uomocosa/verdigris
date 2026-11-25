use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    Ending,
}

impl Phase {
    #[must_use]
    pub const fn steps(self) -> &'static [turns::Step] {
        match self {
            Self::Beginning => &[turns::Step::Untap, turns::Step::Upkeep, turns::Step::Draw],
            Self::PrecombatMain => &[turns::Step::PrecombatMain],
            Self::Combat => &[
                turns::Step::BeginningOfCombat,
                turns::Step::DeclareAttackers,
                turns::Step::DeclareBlockers,
                turns::Step::CombatDamage,
                turns::Step::EndOfCombat,
            ],
            Self::PostcombatMain => &[turns::Step::PostcombatMain],
            Self::Ending => &[turns::Step::End, turns::Step::Cleanup],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::turns;

    use super::Phase;

    #[test]
    fn test_usage() {
        assert_eq!(
            Phase::Beginning.steps(),
            &[turns::Step::Untap, turns::Step::Upkeep, turns::Step::Draw]
        );
        assert_eq!(Phase::PrecombatMain.steps(), &[turns::Step::PrecombatMain]);
        assert_eq!(
            Phase::Combat.steps(),
            &[
                turns::Step::BeginningOfCombat,
                turns::Step::DeclareAttackers,
                turns::Step::DeclareBlockers,
                turns::Step::CombatDamage,
                turns::Step::EndOfCombat,
            ]
        );
        assert_eq!(
            Phase::PostcombatMain.steps(),
            &[turns::Step::PostcombatMain]
        );
        assert_eq!(
            Phase::Ending.steps(),
            &[turns::Step::End, turns::Step::Cleanup]
        );
    }
}
