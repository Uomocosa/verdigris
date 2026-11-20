use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_turn_end_systems(app: &mut App) {
    app.add_systems(
        Update,
        rules::r500_3_r503_1_step_without_priority_ends::r500_3_r503_1_step_without_priority_ends
            .in_set(rules::RuleSet::TurnEnd),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_turn_end_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_turn_end_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(1));
    }
}
