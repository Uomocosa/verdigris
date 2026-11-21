use crate::cards;
use crate::stack;
use crate::testing;
use crate::zones;

pub fn stack_objects(game: &testing::MockGame) -> Vec<testing::ZoneObject> {
    let world = game.app.world();
    world
        .iter_entities()
        .filter(|entity| {
            entity.get::<zones::InZone>().is_some_and(|in_zone| **in_zone == game.stack)
        })
        .filter_map(|entity| {
            if entity.get::<stack::TriggeredAbilityOnStack>().is_some() {
                let source = entity.get::<stack::TriggeredAbilityOnStack>()?.source;
                let definition = world.get::<cards::CardDefinition>(source)?;
                Some(testing::ZoneObject::Trigger {
                    ability: testing::TestTrigger(entity.id()),
                    source_name: definition.name.clone(),
                })
            } else {
                let definition = entity.get::<cards::CardDefinition>()?;
                Some(testing::ZoneObject::Card {
                    card: testing::TestCard(entity.id()),
                    name: definition.name.clone(),
                })
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    use super::stack_objects;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        assert_eq!(stack_objects(&game).len(), 0);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        let objects = stack_objects(&game);
        assert_eq!(objects.len(), 1);
        assert!(matches!(objects[0], ZoneObject::Trigger { .. }));
        Ok(())
    }
}
