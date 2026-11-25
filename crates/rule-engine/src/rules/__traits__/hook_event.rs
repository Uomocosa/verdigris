use bevy::prelude::App;

use crate::HooksDone;
use crate::actions;
use crate::cards;
use crate::rules;
use crate::rules::traits::event::Event;

pub trait HookEvent: Clone + Send + Sync + 'static {
    const ID: rules::EventId;
    fn card(&self) -> cards::CardId;
    #[must_use]
    fn clauses() -> Vec<rules::Clause> {
        Vec::new()
    }
    fn register(app: &mut App) {
        app.add_message::<HooksDone<Self>>();
        <rules::Hooked<Self> as Event>::register(app);
        app.init_resource::<rules::HookQueue<Self>>()
            .insert_resource(actions::PendingMayAsks::<rules::Hooked<Self>>(Vec::new()));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use crate::actions;
    use crate::events;
    use crate::rules;

    use super::HookEvent;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        events::OpeningHandAction::register(&mut app);
        assert!(
            app.world().get_resource::<rules::HookQueue<events::OpeningHandAction>>().is_some()
        );
        assert!(
            app.world()
                .get_resource::<actions::PendingMayAsks<rules::Hooked<events::OpeningHandAction>>>()
                .is_some()
        );
    }
}
