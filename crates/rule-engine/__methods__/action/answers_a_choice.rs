use crate::actions;

pub const fn answers_a_choice(action: &actions::Action) -> bool {
    matches!(
        action,
        actions::Action::Choose(_)
            | actions::Action::OrderTriggers(_)
            | actions::Action::AssignCombatDamage(_)
    )
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::testing::*;

    use super::answers_a_choice;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let legend = game.zone_entity(p1, BATTLEFIELD).unwrap();
        assert!(answers_a_choice(&actions::Action::Choose(vec![legend])));
        assert!(!answers_a_choice(&actions::Action::PassPriority));
    }
}
