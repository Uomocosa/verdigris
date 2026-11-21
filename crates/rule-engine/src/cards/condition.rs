use bevy::prelude::{Entity, Query};

use crate::effects;
use crate::players;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    LifeAtLeast(effects::PlayerRef, i32),
}

impl Condition {
    pub(crate) fn holds(self, controller: Entity, lives: &Query<(Entity, &players::Life)>) -> bool {
        let Self::LifeAtLeast(target, amount) = self;
        let players: Vec<Entity> = lives.iter().map(|(player, _)| player).collect();
        target
            .resolve(controller, &players)
            .iter()
            .all(|player| lives.get(*player).is_ok_and(|(_, life)| **life >= amount))
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;

    use super::Condition;

    #[test]
    fn test_usage() {
        let condition = Condition::LifeAtLeast(YOU, 40);
        assert_eq!(
            condition,
            Condition::LifeAtLeast(effects::PlayerRef::You, 40)
        );
    }
}
