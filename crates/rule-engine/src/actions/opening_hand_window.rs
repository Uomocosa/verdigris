use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::players;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct OpeningHandWindow(pub Vec<players::PlayerId>);

#[rustfmt::skip]
impl OpeningHandWindow {
    #[must_use]
    pub fn is_complete(&self, players: usize) -> bool { self.len() >= players }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::players;

    #[test]
    fn test_usage() {
        let mut window = actions::OpeningHandWindow::default();
        assert!(!window.is_complete(2));
        window.push(players::PlayerId(Entity::PLACEHOLDER));
        window.push(players::PlayerId(Entity::PLACEHOLDER));
        assert!(window.is_complete(2));
    }
}
