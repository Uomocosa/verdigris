use std::any::TypeId;

use bevy::prelude::{Component, Entity, EntityRef};

use crate::rules::traits::entity_component::EntityComponent;

#[derive(Clone, Copy)]
pub struct ComponentRef {
    pub type_id: TypeId,
    pub name: &'static str,
    pub target: Option<for<'a> fn(&'a EntityRef<'a>) -> Option<Entity>>,
}

impl ComponentRef {
    #[must_use]
    pub fn of<C: Component>() -> Self {
        Self {
            type_id: TypeId::of::<C>(),
            name: std::any::type_name::<C>(),
            target: None,
        }
    }

    #[must_use]
    pub fn relation<C: EntityComponent>() -> Self {
        Self {
            type_id: TypeId::of::<C>(),
            name: std::any::type_name::<C>(),
            target: Some(|entity| entity.get::<C>().map(C::target)),
        }
    }
}

#[rustfmt::skip]
impl PartialEq for ComponentRef {
    fn eq(&self, other: &Self) -> bool { self.type_id == other.type_id }
}

impl Eq for ComponentRef {}

#[rustfmt::skip]
impl std::hash::Hash for ComponentRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.type_id.hash(state); }
}

#[rustfmt::skip]
impl std::fmt::Debug for ComponentRef {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "{}", self.name) }
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::ComponentRef;

    #[test]
    fn test_usage() {
        let tapped = ComponentRef::of::<cards::Tapped>();
        assert_eq!(tapped, ComponentRef::of::<cards::Tapped>());
        assert_ne!(tapped, ComponentRef::of::<cards::Creature>());
        assert!(tapped.target.is_none());
        let owner = ComponentRef::relation::<cards::Owner>();
        assert!(owner.target.is_some());
        assert_eq!(format!("{tapped:?}"), tapped.name);
    }
}
