use crate::events;
use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::comprehensive_rules;

pub fn add_step_started_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            comprehensive_rules::r106_4_mana_empties::r106_4_mana_empties,
            comprehensive_rules::r103_8a_r504_1_draw_step::r103_8a_r504_1_draw_step,
        )
            .chain()
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            (
                comprehensive_rules::r502_3_untap::r502_3_untap,
                comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                    events::Untap,
                >,
                comprehensive_rules::r502_3_untap_perform::r502_3_untap_perform,
            )
                .chain(),
            comprehensive_rules::r302_6_summoning_sickness::r302_6_summoning_sickness,
        )
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            comprehensive_rules::r506_2a_r507_1_r508_1b_defending_player::r506_2a_r507_1_r508_1b_defending_player,
            comprehensive_rules::r506_4_r511_3_remove_from_combat::r506_4_r511_3_remove_from_combat,
        )
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            comprehensive_rules::r506_1_r510_4_r702_4b_r702_7b_combat_damage_steps::r506_1_r510_4_r702_4b_r702_7b_combat_damage_steps,
            comprehensive_rules::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_r702_2c_r702_19b_r702_19d_combat_damage::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_r702_2c_r702_19b_r702_19d_combat_damage,
        )
            .chain()
            .in_set(RuleSet::StepStarted),
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

        assert_eq!(
            app.get_schedule(Update).map(Schedule::systems_len),
            Some(10)
        );
    }
}
