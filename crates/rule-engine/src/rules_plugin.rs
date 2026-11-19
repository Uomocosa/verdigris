use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Plugin};

pub struct RulesPlugin;

#[atomic_delegates]
impl Plugin for RulesPlugin {
    fn build(&self, app: &mut App) {}
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
        assert!(
            app.world()
                .get_resource::<actions::PendingChoice>()
                .is_some()
        );
    }
}
