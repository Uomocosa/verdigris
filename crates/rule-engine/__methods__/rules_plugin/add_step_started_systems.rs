use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_step_started_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r106_4_mana_empties::r106_4_mana_empties,
            rules::r103_8a_r504_1_draw_step::r103_8a_r504_1_draw_step,
        )
            .chain()
            .in_set(rules::RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        rules::r502_3_untap::r502_3_untap.in_set(rules::RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            rules::r506_2a_r507_1_r508_1b_defending_player::r506_2a_r507_1_r508_1b_defending_player,
            rules::r506_4_r511_3_remove_from_combat::r506_4_r511_3_remove_from_combat,
        )
            .in_set(rules::RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        rules::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_combat_damage::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_combat_damage.in_set(rules::RuleSet::StepStarted),
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

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(6));
    }
}
