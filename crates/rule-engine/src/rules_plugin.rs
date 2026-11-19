use bevy::prelude::{App, Plugin};

use crate::actions;

pub struct RulesPlugin;

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<actions::PlayerAction>()
            .add_message::<actions::Rejected>();
        app.init_resource::<actions::PendingChoice>();
    }
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
