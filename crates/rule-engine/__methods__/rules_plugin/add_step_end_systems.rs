use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::rules;

pub fn add_step_end_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r500_1_r500_2_step_ends::r500_1_r500_2_step_ends,
            rules::r508_8_skip_combat_steps::r508_8_skip_combat_steps,
            rules::r404_1_graveyard_top::r404_1_graveyard_top,
        )
            .chain()
            .in_set(RuleSet::StepEnd),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_step_end_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_step_end_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(3));
    }
}
