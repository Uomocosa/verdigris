use bevy::ecs::component::ComponentId;
use bevy::prelude::{Component, World};

use crate::rules;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PredicateRegistry {
    entries: [(rules::ComponentKey, Option<ComponentId>); Self::CAPACITY],
    len: usize,
}

impl PredicateRegistry {
    pub const CAPACITY: usize = 32;
}

impl PredicateRegistry {
    pub fn insert(&mut self, key: rules::ComponentKey, component: ComponentId) {
        let existing =
            self.entries.iter_mut().take(self.len).find(|(candidate, _)| *candidate == key);
        if let Some((_, stored)) = existing {
            *stored = Some(component);
            return;
        }
        if let Some((slot_key, stored)) = self.entries.get_mut(self.len) {
            *slot_key = key;
            *stored = Some(component);
            self.len = self.len.saturating_add(1);
        }
    }

    #[must_use]
    pub fn component_id(&self, key: rules::ComponentKey) -> Option<ComponentId> {
        let found = self.entries.iter().take(self.len).find(|(candidate, _)| *candidate == key);
        found.and_then(|(_, component)| *component)
    }

    pub fn register_predicate_component<C>(&mut self, world: &mut World)
    where
        C: rules::traits::keyed_component::KeyedComponent,
    {
        let component = world.register_component::<C>();
        self.insert(C::KEY, component);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use crate::cards;
    use crate::rules;
    use crate::rules::traits::keyed_component::KeyedComponent;

    use super::PredicateRegistry;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let mut registry = PredicateRegistry::default();
        registry.register_predicate_component::<cards::Tapped>(app.world_mut());
        registry.register_predicate_component::<cards::Flying>(app.world_mut());

        assert_eq!(
            registry.component_id(cards::Tapped::KEY),
            registry.component_id(rules::ComponentKey("tapped"))
        );
        assert!(registry.component_id(cards::Tapped::KEY).is_some());
        assert!(registry.component_id(cards::Flying::KEY).is_some());
        assert!(registry.component_id(rules::ComponentKey("reach")).is_none());
        let singleton = app.world_mut().spawn((registry,)).id();
        assert!(app.world().get::<PredicateRegistry>(singleton).is_some());
    }
}
