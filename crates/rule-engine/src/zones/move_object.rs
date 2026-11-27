use bevy::prelude::{Commands, Entity};

use crate::cards;
use crate::damage;
use crate::effects;
use crate::stack;
use crate::zones;

pub fn move_object(
    commands: &mut Commands,
    old: Entity,
    definition: cards::CardDefinition,
    owner: cards::Owner,
    token: Option<cards::Token>,
    to: Entity,
) -> Entity {
    let new = commands.spawn((definition, owner, damage::Damage(0), zones::InZone(to))).id();
    if token.is_some() {
        commands.entity(new).insert(cards::Token);
    }
    commands
        .entity(old)
        .remove::<zones::InZone>()
        .insert(zones::LeftZone(new))
        .remove::<cards::OnBattlefield>()
        .remove::<cards::Creature>()
        .remove::<cards::Attacking>()
        .remove::<cards::Blocking>()
        .remove::<cards::Blocked>()
        .remove::<cards::SummoningSick>()
        .remove::<cards::HadFirstStrikeAsDamageBegan>()
        .remove::<cards::TriggeredEnters>()
        .remove::<cards::TriggeredAtStep>()
        .remove::<stack::CastingStage>()
        .remove::<stack::ChosenTarget>()
        .remove::<effects::SourceTimestamp>();
    new
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, Entity};

    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::cards;
    use crate::testing::*;
    use crate::zones;

    use super::move_object;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let graveyard = game.zone_entity(p1, GRAVEYARD).unwrap();
        let definition = game.get::<cards::CardDefinition>(*bears).unwrap();
        let owner = game.get::<cards::Owner>(*bears).unwrap();
        let old = Entity::from(bears);
        game.app
            .world_mut()
            .run_system_once(move |mut commands: Commands| {
                move_object(
                    &mut commands,
                    old,
                    definition.clone(),
                    owner,
                    None,
                    graveyard,
                );
            })
            .unwrap();
        assert!(game.has::<zones::LeftZone>(old));
        assert!(!game.has::<zones::InZone>(old));
        let [moved] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(GRIZZLY_BEARS)?;
        assert_eq!(
            game.get::<zones::LeftZone>(old),
            Some(zones::LeftZone(Entity::from(moved)))
        );
        assert!(!game.has::<cards::Token>(*moved));
        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        assert!(!game.card(bears).is_in(GRAVEYARD).of(p1));
        assert!(!game.card(bears).is_in(BATTLEFIELD).of(p1));
        Ok(())
    }
}
