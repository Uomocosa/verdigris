use bevy::prelude::{Entity, Resource};
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct OpeningHandWindow(pub Vec<Entity>);

#[rustfmt::skip]
impl OpeningHandWindow {
    #[must_use]
    pub fn is_complete(&self, players: usize) -> bool { self.len() >= players }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;

    #[test]
    fn test_usage() {
        let mut window = actions::OpeningHandWindow::default();
        assert!(!window.is_complete(2));
        window.push(Entity::PLACEHOLDER);
        window.push(Entity::PLACEHOLDER);
        assert!(window.is_complete(2));
    }
}
