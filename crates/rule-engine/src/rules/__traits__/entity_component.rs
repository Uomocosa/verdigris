use bevy::prelude::{Component, Entity};

pub trait EntityComponent: Component {
    fn target(&self) -> Entity;
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::players;
    use crate::rules::traits::entity_component::EntityComponent;

    #[test]
    fn test_usage() {
        let owner = cards::Owner(players::PlayerId(bevy::prelude::Entity::PLACEHOLDER));
        assert_eq!(owner.target(), bevy::prelude::Entity::PLACEHOLDER);
    }
}
