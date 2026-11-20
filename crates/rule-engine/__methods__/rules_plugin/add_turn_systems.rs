use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::rules;

pub fn add_turn_systems(app: &mut App) {
    app.add_systems(
        Update,
        rules::r117_3a_r117_3c_r117_3d_priority::r117_3a_r117_3c_r117_3d_priority
            .in_set(RuleSet::Turn),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_turn_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_turn_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(1));
    }
}
