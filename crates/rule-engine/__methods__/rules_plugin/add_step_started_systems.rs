use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_step_started_systems(app: &mut App) {
    app.add_systems(
        Update,
        rules::r504_1_draw_step::r504_1_draw_step.in_set(rules::RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        rules::r502_3_untap::r502_3_untap.in_set(rules::RuleSet::StepStarted),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_step_started_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_step_started_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(2));
    }
}
