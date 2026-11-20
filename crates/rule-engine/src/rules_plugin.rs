use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Plugin};

pub struct RulesPlugin;

#[atomic_delegates]
impl Plugin for RulesPlugin {
    fn build(&self, app: &mut App) {}
}

#[atomic_delegates]
impl RulesPlugin {
    pub fn register_messages_and_resources(app: &mut App) {}

    pub fn configure_schedule_sets(app: &mut App) {}

    pub fn add_layers_systems(app: &mut App) {}

    pub fn add_turn_systems(app: &mut App) {}

    pub fn add_priority_action_systems(app: &mut App) {}

    pub fn add_resolution_systems(app: &mut App) {}

    pub fn add_state_based_systems(app: &mut App) {}

    pub fn add_step_end_systems(app: &mut App) {}

    pub fn add_step_started_systems(app: &mut App) {}

    pub fn add_cleanup_systems(app: &mut App) {}

    pub fn add_turn_end_systems(app: &mut App) {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use crate::actions;

    use super::RulesPlugin;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        app.add_plugins(RulesPlugin);
        assert!(app.world().get_resource::<actions::PendingChoice>().is_some());
    }
}
