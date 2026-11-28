use bevy::prelude::{Component, Entity};

pub trait EntityComponent: Component {
    fn targets(&self) -> Vec<Entity>;
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::players;
    use crate::rules::traits::entity_component::EntityComponent;

    #[test]
    fn test_usage() {
        let owner = cards::Owner(players::PlayerId(bevy::prelude::Entity::PLACEHOLDER));
        assert_eq!(owner.targets(), vec![bevy::prelude::Entity::PLACEHOLDER]);
    }
}
