use crate::testing;

pub fn from(
    targets: &testing::ExpectedTargets,
    new_targets: impl IntoIterator<Item = bevy::prelude::Entity>,
) -> testing::ExpectedTargets {
    testing::ExpectedTargets {
        count: targets.count,
        legal: new_targets.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::prelude::World;

    use crate::testing::*;

    use super::from;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();
        let targets = from(&pick(1), [first, second]);
        assert_eq!(targets.count, 1);
        assert_eq!(targets.legal, BTreeSet::from([first, second]));
    }

    #[test]
    fn pick_from_accepts_an_array_of_targets() {
        let mut world = World::new();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();
        let targets = pick(1).from([first, second]);
        assert_eq!(targets.count, 1);
        assert_eq!(targets.legal, BTreeSet::from([first, second]));
    }
}
