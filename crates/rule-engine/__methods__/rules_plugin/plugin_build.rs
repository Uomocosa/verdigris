use bevy::prelude::App;

use crate::RulesPlugin;

pub fn plugin_build(_rules_plugin: &RulesPlugin, app: &mut App) {
    RulesPlugin::register_messages_and_resources(app);
    RulesPlugin::configure_schedule_sets(app);
    RulesPlugin::add_turn_systems(app);
    RulesPlugin::add_priority_action_systems(app);
    RulesPlugin::add_resolution_systems(app);
    RulesPlugin::add_state_based_systems(app);
    RulesPlugin::add_step_end_systems(app);
    RulesPlugin::add_step_started_systems(app);
    RulesPlugin::add_cleanup_systems(app);
    RulesPlugin::add_turn_end_systems(app);
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
