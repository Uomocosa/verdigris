use bevy::prelude::Entity;

use crate::cards;
use crate::events;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_enchanted<E: EventArgs>(
    answers: &rules::Rules,
    event: &E,
    viewpoint: Option<Entity>,
) -> bool {
    let Some(rules::ArgValue::Entity(source)) =
        answers.resolve(rules::ArgRef::Arg(events::AbilityContext::SOURCE), event)
    else {
        return false;
    };
    viewpoint.is_some_and(|subject| {
        answers
            .entities
            .get(source)
            .ok()
            .and_then(|entity| entity.get::<cards::Enchanting>())
            .is_some_and(|enchanting| **enchanting == subject)
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::events;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;

    use super::eval_enchanted;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([HOLY_STRENGTH]).mana_pool(mana![W]),
            P2.battlefield([GRIZZLY_BEARS, WIND_DRAKE]),
        ]);
        let [strength] = game.player(p1).cards_in(HAND).exactly::<1>(HOLY_STRENGTH)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.play([
            p1.cast(strength).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(strength).to(BATTLEFIELD).of(p1));
        let [aura] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(HOLY_STRENGTH)?;
        let context = events::AbilityContext {
            ability: *aura,
            source: *aura,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_enchanted(&answers, &context, Some(*bears)));
                assert!(!eval_enchanted(&answers, &context, Some(*drake)));
                assert!(!eval_enchanted(&answers, &context, None));
            })
            .unwrap();
        Ok(())
    }
}
