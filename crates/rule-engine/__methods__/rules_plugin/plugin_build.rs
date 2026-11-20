use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RulesPlugin;
use crate::actions;
use crate::damage;
use crate::effects;
use crate::players;
use crate::rules;
use crate::stack;
use crate::turns;

pub fn plugin_build(_rules_plugin: &RulesPlugin, app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<stack::Resolution>()
        .add_message::<stack::Countered>()
        .add_message::<damage::DamageDealt>()
        .add_message::<damage::Destruction>()
        .add_message::<players::DrawRequested>()
        .add_message::<players::LifeLost>()
        .add_message::<players::DiscardRequested>()
        .add_message::<effects::PowerToughnessModified>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::AllPlayersPassed>();
    app.init_resource::<actions::PendingChoice>();
    app.configure_sets(
        Update,
        (
            rules::RuleSet::Turn,
            rules::RuleSet::Casting,
            rules::RuleSet::Resolution,
            rules::RuleSet::StateBased,
            rules::RuleSet::StepEnd,
            rules::RuleSet::StepStarted,
            rules::RuleSet::Cleanup,
            rules::RuleSet::TurnEnd,
        )
            .chain(),
    );
    app.add_systems(
        Update,
        rules::r117_3_priority::r117_3_priority.in_set(rules::RuleSet::Turn),
    );
    app.add_systems(
        Update,
        (
            rules::r601_2a_cast::r601_2a_cast,
            rules::r601_2c_targets::r601_2c_targets,
            rules::r601_2h_pay::r601_2h_pay,
            rules::r508_1a_declare_attackers::r508_1a_declare_attackers,
        )
            .chain()
            .in_set(rules::RuleSet::Casting),
    );
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
    app.add_systems(
        Update,
        (
            rules::r704_5g_lethal_damage::r704_5g_lethal_damage,
            rules::r704_5h_deathtouch_damage::r704_5h_deathtouch_damage,
            rules::r702_12b_indestructible::r702_12b_indestructible,
            rules::r704_5a_zero_life::r704_5a_zero_life,
            rules::r704_5b_empty_library::r704_5b_empty_library,
        )
            .chain()
            .in_set(rules::RuleSet::StateBased),
    );
    app.add_systems(
        Update,
        rules::r500_2_step_ends::r500_2_step_ends.in_set(rules::RuleSet::StepEnd),
    );
    app.add_systems(
        Update,
        rules::r504_1_draw_step::r504_1_draw_step.in_set(rules::RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        rules::r514_2_cleanup_step::r514_2_cleanup_step.in_set(rules::RuleSet::Cleanup),
    );
    app.add_systems(
        Update,
        rules::r500_3_step_without_priority_ends::r500_3_step_without_priority_ends
            .in_set(rules::RuleSet::TurnEnd),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use crate::RulesPlugin;
    use crate::actions;

    use super::plugin_build;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        plugin_build(&RulesPlugin, &mut app);
        assert!(
            app.world()
                .get_resource::<actions::PendingChoice>()
                .is_some()
        );
    }
}
