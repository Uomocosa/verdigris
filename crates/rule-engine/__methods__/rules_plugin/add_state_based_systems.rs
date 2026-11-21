use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::StateBasedActions;
use crate::rules;

pub fn add_state_based_systems(app: &mut App) {
    app.add_systems(
        StateBasedActions,
        (
            rules::r613_1_layers::r613_1_layers,
            rules::r704_5f_zero_toughness::r704_5f_zero_toughness,
            rules::r704_5g_lethal_damage::r704_5g_lethal_damage,
            rules::r704_5h_deathtouch_damage::r704_5h_deathtouch_damage,
            rules::r704_5j_legend_rule::r704_5j_legend_rule,
            rules::r702_12b_indestructible::r702_12b_indestructible,
            rules::r704_5a_zero_life::r704_5a_zero_life,
            rules::r704_5b_empty_library::r704_5b_empty_library,
            rules::r104_2a_wins_when_opponents_left::r104_2a_wins_when_opponents_left,
            rules::r104_4a_draw::r104_4a_draw,
        )
            .chain(),
    );
    app.add_systems(
        Update,
        (
            rules::r603_2_r603_6a_triggered_abilities::r603_2_r603_6a_triggered_abilities,
            rules::r603_4_intervening_if::r603_4_intervening_if,
            rules::r603_3_triggers_on_stack::r603_3_triggers_on_stack,
            rules::r117_5_r704_3_state_based_actions::r117_5_r704_3_state_based_actions,
            rules::r603_3b_apnap_trigger_order::r603_3b_apnap_trigger_order,
        )
            .chain()
            .in_set(RuleSet::StateBased),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use crate::StateBasedActions;

    use super::add_state_based_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_state_based_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(5));
        assert_eq!(
            app.get_schedule(StateBasedActions).map(Schedule::systems_len),
            Some(10)
        );
    }
}
