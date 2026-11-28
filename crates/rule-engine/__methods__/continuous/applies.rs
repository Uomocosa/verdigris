use bevy::prelude::Entity;

use crate::abilities;
use crate::events;
use crate::rules;

pub fn applies(
    ability: &abilities::Continuous,
    answers: &rules::Rules,
    source: Entity,
    affected: Entity,
) -> bool {
    let context = events::AbilityContext {
        spell_or_ability: source,
    };
    let condition_holds =
        ability.as_long_as.as_ref().is_none_or(|condition| answers.eval(condition, &context, None));
    condition_holds && answers.eval(&ability.filter, &context, Some(affected))
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::YOU;
    use crate::creature;
    use crate::keywords;
    use crate::rules;
    use crate::statics;
    use crate::testing::*;

    use super::applies;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let ability = statics::all(creature()).gain(keywords::vigilance());
                assert!(applies(&ability, &answers, *bears, *bears));
                let gated =
                    ability.as_long_as(rules::life().of(YOU).is(rules::Value(30).or_more()));
                assert!(!applies(&gated, &answers, *bears, *bears));
                let narrow = statics::all(!creature()).gain(keywords::vigilance());
                assert!(!applies(&narrow, &answers, *bears, *bears));
            })
            .unwrap();
        Ok(())
    }
}
