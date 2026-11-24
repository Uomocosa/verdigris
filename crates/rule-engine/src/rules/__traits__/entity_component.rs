use bevy::prelude::Entity;

use crate::rules;

pub trait EntityComponent: rules::traits::keyed_component::KeyedComponent {
    fn target(&self) -> Entity;
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::entity_component::EntityComponent;
    use crate::rules::traits::keyed_component::KeyedComponent;

    #[test]
    fn test_usage() {
        assert_eq!(cards::Owner::KEY, rules::ComponentKey("owner"));
        assert_eq!(cards::Blocking::KEY, rules::ComponentKey("blocking"));
        assert_eq!(cards::Enchanting::KEY, rules::ComponentKey("enchanting"));
        let owner = cards::Owner(players::PlayerId(bevy::prelude::Entity::PLACEHOLDER));
        assert_eq!(owner.target(), bevy::prelude::Entity::PLACEHOLDER);
    }
}
