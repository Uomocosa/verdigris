use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::rules;

pub fn add_state_based_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            rules::r704_5f_zero_toughness::r704_5f_zero_toughness,
            rules::r704_5g_lethal_damage::r704_5g_lethal_damage,
            rules::r704_5h_deathtouch_damage::r704_5h_deathtouch_damage,
            rules::r702_12b_indestructible::r702_12b_indestructible,
            rules::r704_5a_zero_life::r704_5a_zero_life,
            rules::r704_5b_empty_library::r704_5b_empty_library,
        )
            .chain()
            .in_set(rules::RuleSet::StateBased),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_state_based_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_state_based_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(6));
    }
}
