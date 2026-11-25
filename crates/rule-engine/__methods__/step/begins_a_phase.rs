use crate::turns;

pub const fn begins_a_phase(step: turns::Step) -> bool {
    matches!(
        step,
        turns::Step::Untap
            | turns::Step::PrecombatMain
            | turns::Step::BeginningOfCombat
            | turns::Step::PostcombatMain
            | turns::Step::End
    )
}

#[cfg(test)]
mod tests {
    use crate::turns;

    use super::begins_a_phase;

    #[test]
    fn test_usage() {
        assert!(begins_a_phase(turns::Step::Untap));
        assert!(begins_a_phase(turns::Step::PrecombatMain));
        assert!(begins_a_phase(turns::Step::BeginningOfCombat));
        assert!(begins_a_phase(turns::Step::PostcombatMain));
        assert!(begins_a_phase(turns::Step::End));
        assert!(!begins_a_phase(turns::Step::Upkeep));
        assert!(!begins_a_phase(turns::Step::Draw));
        assert!(!begins_a_phase(turns::Step::DeclareAttackers));
        assert!(!begins_a_phase(turns::Step::CombatDamage));
        assert!(!begins_a_phase(turns::Step::EndOfCombat));
        assert!(!begins_a_phase(turns::Step::Cleanup));
    }
}
