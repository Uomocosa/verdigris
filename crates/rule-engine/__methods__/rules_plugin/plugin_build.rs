use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RulesPlugin;
use crate::actions;
use crate::rules;
use crate::stack;
use crate::zones;

pub fn plugin_build(_rules_plugin: &RulesPlugin, app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<stack::Resolution>()
        .add_message::<zones::DamageDealt>()
        .add_message::<zones::Destruction>();
    app.init_resource::<actions::PendingChoice>();
    app.configure_sets(
        Update,
        (
            rules::RuleSet::Casting,
            rules::RuleSet::Resolution,
            rules::RuleSet::StateBased,
        )
            .chain(),
    );
    app.add_systems(
        Update,
        (
            rules::r601_2a_cast::r601_2a_cast,
            rules::r601_2c_targets::r601_2c_targets,
            rules::r601_2h_pay::r601_2h_pay,
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
            rules::r120_3_damage_results::r120_3_damage_results,
            rules::r608_2n_graveyard::r608_2n_graveyard,
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
        )
            .chain()
            .in_set(rules::RuleSet::StateBased),
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
