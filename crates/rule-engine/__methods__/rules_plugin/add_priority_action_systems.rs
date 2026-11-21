use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::rules;

pub fn add_priority_action_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r601_2a_cast::r601_2a_cast,
            rules::r601_2c_targets::r601_2c_targets,
            rules::r601_2h_pay::r601_2h_pay,
            rules::r508_1a_r508_1k_declare_attackers::r508_1a_r508_1k_declare_attackers,
            rules::r508_1f_tap_attackers::r508_1f_tap_attackers,
            rules::r509_1a_r509_1g_r509_1h_r702_9b_r702_17b_declare_blockers::r509_1a_r509_1g_r509_1h_r702_9b_r702_17b_declare_blockers,
            rules::r305_1_r305_2_r305_3_play_land::r305_1_r305_2_r305_3_play_land,
            rules::r305_6_basic_land_mana_ability::r305_6_basic_land_mana_ability,
            rules::r601_2g_mana_abilities::r601_2g_mana_abilities,
        )
            .chain()
            .in_set(RuleSet::PriorityAction),
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

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(9));
    }
}
