use crate::events;
use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::Attempt;
use crate::RuleSet;
use crate::comprehensive_rules;

pub fn add_priority_action_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            comprehensive_rules::r601_2a_cast::r601_2a_cast(),
            comprehensive_rules::r601_2c_targets::r601_2c_targets(),
            comprehensive_rules::r119_4_r119_8_pay_life::r119_4_r119_8_pay_life,
            comprehensive_rules::r601_2h_pay::r601_2h_pay,
            comprehensive_rules::r508_1a_declare_attackers::r508_1a_declare_attackers,
            Attempt::<events::DeclareAttackers>::resolved_by(
                comprehensive_rules::r508_1k_becomes_attacking::r508_1k_becomes_attacking,
            ),
            comprehensive_rules::r508_1m_r508_2a_r508_3a_attack_triggers::r508_1m_r508_2a_r508_3a_attack_triggers,
            comprehensive_rules::r611_3a_static_effects::r611_3a_static_effects,
            (
                comprehensive_rules::r508_1f_tap_attackers::r508_1f_tap_attackers(),
                Attempt::<events::Tap>::resolved_by(
                    comprehensive_rules::r701_26a_tap::r701_26a_tap,
                ),
            )
                .chain(),
            (
                comprehensive_rules::r509_1a_declare_blockers::r509_1a_declare_blockers,
                Attempt::<events::DeclareBlocker>::resolved_by(
                    comprehensive_rules::r509_1g_becomes_blocking::r509_1g_becomes_blocking,
                ),
                comprehensive_rules::r509_1h_becomes_blocked::r509_1h_becomes_blocked,
            )
                .chain(),
            comprehensive_rules::r509_1i_r509_2a_r509_3a_r509_3c_blocker_triggers::r509_1i_r509_2a_r509_3a_r509_3c_blocker_triggers,
            comprehensive_rules::r305_1_r305_2_r305_3_play_land::r305_1_r305_2_r305_3_play_land(),
            comprehensive_rules::r305_6_basic_land_mana_ability::r305_6_basic_land_mana_ability,
            comprehensive_rules::r605_3a_mana_abilities::r605_3a_mana_abilities,
            comprehensive_rules::r601_2g_mana_abilities::r601_2g_mana_abilities,
            comprehensive_rules::r104_3a_concede::r104_3a_concede,
            comprehensive_rules::r104_3_lose_the_game::r104_3_lose_the_game,
            comprehensive_rules::r104_5_leaves_the_game::r104_5_leaves_the_game,
            comprehensive_rules::r800_4a_priority_of_a_player_who_left::r800_4a_priority_of_a_player_who_left,
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

        assert_eq!(
            app.get_schedule(Update).map(Schedule::systems_len),
            Some(30)
        );
    }
}
