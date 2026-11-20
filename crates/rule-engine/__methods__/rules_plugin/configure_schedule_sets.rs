use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn configure_schedule_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (
            rules::RuleSet::Turn,
            rules::RuleSet::PriorityAction,
            rules::RuleSet::Resolution,
            rules::RuleSet::StateBased,
            rules::RuleSet::StepEnd,
            rules::RuleSet::StepStarted,
            rules::RuleSet::Cleanup,
            rules::RuleSet::TurnEnd,
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
