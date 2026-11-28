use std::collections::BTreeSet;

use bevy::prelude::{Component, Entity};

use crate::rules;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Blocking {
    One(Entity),
    Many(BTreeSet<Entity>),
}

impl Blocking {
    #[must_use]
    pub fn from_attackers(attackers: BTreeSet<Entity>) -> Option<Self> {
        let first = attackers.iter().next().copied()?;
        if attackers.len() == 1 {
            Some(Self::One(first))
        } else {
            Some(Self::Many(attackers))
        }
    }
}

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Blocking {
    fn targets(&self) -> Vec<Entity> { self.attackers() }
}

impl Blocking {
    #[must_use]
    pub fn attackers(&self) -> Vec<Entity> {
        match self {
            Self::One(attacker) => vec![*attacker],
            Self::Many(attackers) => attackers.iter().copied().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::prelude::Entity;

    use crate::cards;
    use crate::rules::traits::entity_component::EntityComponent;

    #[test]
    fn test_usage() {
        let first = Entity::from_raw_u32(1).unwrap();
        let second = Entity::from_raw_u32(2).unwrap();
        let one = cards::Blocking::from_attackers(BTreeSet::from([first])).unwrap();
        assert_eq!(one.attackers(), vec![first]);
        assert_eq!(one.targets(), vec![first]);
        let many = cards::Blocking::from_attackers(BTreeSet::from([first, second])).unwrap();
        assert_eq!(many.attackers().len(), 2);
        assert!(many.attackers().contains(&first) && many.attackers().contains(&second));
        assert_eq!(many.targets(), many.attackers());
        assert!(cards::Blocking::from_attackers(BTreeSet::new()).is_none());
    }
}
