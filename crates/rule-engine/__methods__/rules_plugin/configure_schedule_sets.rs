use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;

pub fn configure_schedule_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (
            RuleSet::Turn,
            RuleSet::PriorityAction,
            RuleSet::Resolution,
            RuleSet::StateBased,
            RuleSet::StepEnd,
            RuleSet::StepStarted,
            RuleSet::Cleanup,
            RuleSet::TurnEnd,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Update};

    use super::configure_schedule_sets;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        configure_schedule_sets(&mut app);

        assert!(app.get_schedule(Update).is_some());
    }
}
