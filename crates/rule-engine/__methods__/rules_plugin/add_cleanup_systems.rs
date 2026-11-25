use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::comprehensive_rules;
use crate::turns;

pub fn add_cleanup_systems(app: &mut App) {
    app.init_resource::<turns::CleanupReentry>();
    app.add_systems(
        Update,
        (
            comprehensive_rules::r514_1_cleanup_discard::r514_1_cleanup_discard(),
            comprehensive_rules::r514_2_cleanup_step::r514_2_cleanup_step(),
            comprehensive_rules::r514_3a_cleanup_reentry::r514_3a_cleanup_reentry(),
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

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(4));
    }
}
