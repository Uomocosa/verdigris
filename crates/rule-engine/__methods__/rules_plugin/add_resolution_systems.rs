use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_resolution_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r117_4_priority::r117_4_priority,
            rules::r608_2b_illegal_targets::r608_2b_illegal_targets,
            rules::r608_2_resolve::r608_2_resolve,
            rules::r611_2a_continuous_effects::r611_2a_continuous_effects,
            rules::r121_1_draw::r121_1_draw,
            rules::r119_3_life_loss::r119_3_life_loss,
            rules::r701_9_discard::r701_9_discard,
            rules::r701_6_counter::r701_6_counter,
            rules::r120_3_damage_results::r120_3_damage_results,
            rules::r608_2n_graveyard::r608_2n_graveyard,
            rules::r608_3a_permanent_enters::r608_3a_permanent_enters,
        )
            .chain()
            .in_set(rules::RuleSet::Resolution),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_resolution_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_resolution_systems(&mut app);

        assert_eq!(
            app.get_schedule(Update).map(Schedule::systems_len),
            Some(11)
        );
    }
}
