use bevy::prelude::Entity;
use std::collections::BTreeSet;

use crate::effects;
use crate::players;
use crate::stack;

pub fn legal_targets(
    targeting: &stack::Targeting,
    spell: Entity,
    requirement: effects::Target,
    controller: players::PlayerId,
) -> BTreeSet<Entity> {
    let mut legal = BTreeSet::new();
    if matches!(
        requirement.filter.kind,
        effects::FilterKind::AnyTarget | effects::FilterKind::Player
    ) {
        for player in targeting.players.iter() {
            if targeting.is_legal_target(spell, requirement, controller, player) {
                legal.insert(player);
            }
        }
    }
    for (candidate, ..) in targeting.cards.iter() {
        if targeting.is_legal_target(spell, requirement, controller, candidate) {
            legal.insert(candidate);
        }
    }
    legal
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;
    use std::collections::BTreeSet;

    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{ANY_TARGET, BATTLEFIELD, HAND};

    use super::legal_targets;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |targeting: stack::Targeting| {
                assert_eq!(
                    legal_targets(&targeting, *bolt, effects::target(1, ANY_TARGET), p1.id),
                    BTreeSet::from([*p1, *p2, Entity::from(bears)])
                );
            })
            .unwrap();
        Ok(())
    }
}
