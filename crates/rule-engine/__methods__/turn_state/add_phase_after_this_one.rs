use crate::turns;

pub fn add_phase_after_this_one(state: &mut turns::TurnState, phase: turns::Phase) {
    let steps = phase.steps();
    match state.upcoming_steps.iter().position(|step| step.begins_a_phase()) {
        Some(index) => {
            for (offset, step) in steps.iter().enumerate() {
                state.upcoming_steps.insert(index.saturating_add(offset), *step);
            }
        }
        None => state.upcoming_steps.extend(steps.iter().copied()),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::turns;

    use super::add_phase_after_this_one;

    #[test]
    fn test_usage() {
        let mut state = turns::TurnState::from(turns::Turn {
            number: 2,
            active: players::PlayerId(Entity::PLACEHOLDER),
            step: turns::Step::PostcombatMain,
        });
        add_phase_after_this_one(&mut state, turns::Phase::Combat);
        assert_eq!(state.next_step(), Some(turns::Step::BeginningOfCombat));
        add_phase_after_this_one(&mut state, turns::Phase::Beginning);
        assert_eq!(
            state.next_step(),
            Some(turns::Step::Untap),
            "the most recently created phase occurs first"
        );
    }
}
