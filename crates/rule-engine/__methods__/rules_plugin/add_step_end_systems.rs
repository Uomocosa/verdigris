use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::comprehensive_rules;

pub fn add_step_end_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            comprehensive_rules::r500_2_step_ends::r500_2_step_ends(),
            comprehensive_rules::r103_8a_skip_first_draw_step::r103_8a_skip_first_draw_step,
            comprehensive_rules::r508_8_skip_combat_steps::r508_8_skip_combat_steps,
            comprehensive_rules::r500_1_steps_proceed_in_order::r500_1_steps_proceed_in_order,
            comprehensive_rules::r404_1_graveyard_top::r404_1_graveyard_top,
            comprehensive_rules::r404_3_graveyard_order::r404_3_graveyard_order,
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

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(6));
    }
}
