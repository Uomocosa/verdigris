use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_priority_action_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r601_2a_cast::r601_2a_cast,
            rules::r601_2c_targets::r601_2c_targets,
            rules::r601_2h_pay::r601_2h_pay,
            rules::r508_1a_declare_attackers::r508_1a_declare_attackers,
            rules::r508_1f_tap_attackers::r508_1f_tap_attackers,
            rules::r509_1a_declare_blockers::r509_1a_declare_blockers,
        )
            .chain()
            .in_set(rules::RuleSet::PriorityAction),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_priority_action_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_priority_action_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(6));
    }
}
