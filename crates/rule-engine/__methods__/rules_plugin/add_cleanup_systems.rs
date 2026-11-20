use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::rules;

pub fn add_cleanup_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r514_1_cleanup_discard::r514_1_cleanup_discard,
            rules::r514_2_cleanup_step::r514_2_cleanup_step,
        )
            .chain()
            .in_set(RuleSet::Cleanup),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_cleanup_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_cleanup_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(2));
    }
}
