use bevy::prelude::{App, IntoScheduleConfigs, Update, not, resource_exists};

use crate::GameEnded;
use crate::RuleSet;

pub fn configure_schedule_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (
            RuleSet::Turn.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::PriorityAction.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::Resolution.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::StateBased.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::StepEnd.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::StepStarted.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::Cleanup.run_if(not(resource_exists::<GameEnded>)),
            RuleSet::TurnEnd.run_if(not(resource_exists::<GameEnded>)),
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
