use crate::events;
use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::Attempt;
use crate::RuleSet;
use crate::comprehensive_rules;

pub fn add_step_started_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            comprehensive_rules::r500_4_mana_pools_empty::r500_4_mana_pools_empty(),
            Attempt::<events::LoseUnspentMana>::resolved_by(
                comprehensive_rules::r106_4_lose_unspent_mana::r106_4_lose_unspent_mana,
            ),
            comprehensive_rules::r504_1_draw_step::r504_1_draw_step(),
        )
            .chain()
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            (
                comprehensive_rules::r502_3_untap::r502_3_untap(),
                Attempt::<events::Untap>::resolved_by(
                    comprehensive_rules::r701_26b_untap::r701_26b_untap,
                ),
            )
                .chain(),
            comprehensive_rules::r302_6_summoning_sickness::r302_6_summoning_sickness(),
        )
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            (
                comprehensive_rules::r506_2_defending_player::r506_2_defending_player(),
                comprehensive_rules::r507_1_choose_defending_player::r507_1_choose_defending_player(
                ),
            )
                .chain(),
            comprehensive_rules::r506_4_leaves_the_battlefield::r506_4_leaves_the_battlefield,
            comprehensive_rules::r511_3_end_of_combat_ends::r511_3_end_of_combat_ends(),
        )
            .in_set(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        (
            comprehensive_rules::r510_4_combat_damage_steps::r510_4_combat_damage_steps(),
            comprehensive_rules::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_combat_damage::r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_combat_damage(),
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
            Some(17)
        );
    }
}
