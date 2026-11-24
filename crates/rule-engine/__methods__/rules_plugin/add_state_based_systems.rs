use bevy::ecs::schedule::ApplyDeferred;
use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::StateBasedActions;
use crate::comprehensive_rules;
use crate::rules;

pub fn add_state_based_systems(app: &mut App) {
    app.add_systems(
        StateBasedActions,
        (
            (
                comprehensive_rules::r611_3a_static_effects::r611_3a_static_effects,
                comprehensive_rules::r613_1_layers::r613_1_layers,
            )
                .chain(),
            ApplyDeferred,
            (
                (
                    comprehensive_rules::r704_5f_zero_toughness::r704_5f_zero_toughness,
                    comprehensive_rules::r704_5g_lethal_damage::r704_5g_lethal_damage,
                    comprehensive_rules::r704_5h_deathtouch_damage::r704_5h_deathtouch_damage,
                    comprehensive_rules::r704_5j_legend_rule::r704_5j_legend_rule,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::Destroy,
                    >,
                    comprehensive_rules::r702_12b_indestructible::r702_12b_indestructible,
                    comprehensive_rules::r704_5m_aura_illegal::r704_5m_aura_illegal,
                    comprehensive_rules::r704_5d_token_outside_battlefield::r704_5d_token_outside_battlefield,
                    comprehensive_rules::r704_5a_zero_life::r704_5a_zero_life,
                    comprehensive_rules::r704_5b_empty_library::r704_5b_empty_library,
                    comprehensive_rules::r704_5c_poison::r704_5c_poison,
                    comprehensive_rules::r104_2a_wins_when_opponents_left::r104_2a_wins_when_opponents_left,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::DrawFromEmptyLibrary,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::LoseGame,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::WinGame,
                    >,
                    comprehensive_rules::r121_4_draw_from_empty_library::r121_4_draw_from_empty_library,
                )
                    .chain(),
                (
                    comprehensive_rules::r104_3_lose_the_game::r104_3_lose_the_game,
                    comprehensive_rules::r104_2_win_the_game::r104_2_win_the_game,
                    comprehensive_rules::r104_4a_draw::r104_4a_draw,
                    comprehensive_rules::r104_5_leaves_the_game::r104_5_leaves_the_game,
                    comprehensive_rules::r104_1_game_ends::r104_1_game_ends,
                )
                    .chain(),
            )
                .chain(),
        )
            .chain(),
    );
    app.add_systems(
        Update,
        (
            comprehensive_rules::r603_2_r603_6a_triggered_abilities::r603_2_r603_6a_triggered_abilities,
            comprehensive_rules::r603_6c_r700_4_leaves_the_battlefield::r603_6c_r700_4_leaves_the_battlefield,
            comprehensive_rules::r603_4_intervening_if::r603_4_intervening_if,
            comprehensive_rules::r603_3_triggers_on_stack::r603_3_triggers_on_stack,
            comprehensive_rules::r117_5_r704_3_state_based_actions::r117_5_r704_3_state_based_actions,
            comprehensive_rules::r800_4a_priority_of_a_player_who_left::r800_4a_priority_of_a_player_who_left,
            comprehensive_rules::r603_3b_apnap_trigger_order::r603_3b_apnap_trigger_order,
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

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(7));
        assert_eq!(
            app.get_schedule(StateBasedActions).map(Schedule::systems_len),
            Some(24)
        );
    }
}
