use bevy::prelude::Entity;

use crate::cards;
use crate::effects;
use crate::rules;

pub fn characteristics(answers: &rules::Rules, entity: Entity) -> cards::Characteristics {
    let mut applicable: Vec<effects::ContinuousEffect> =
        answers.effects.iter().filter(|effect| effect.affected == entity).copied().collect();
    applicable.sort_by_key(|effect| effect.timestamp);
    answers
        .definitions
        .get(entity)
        .map_or(
            cards::Characteristics {
                on_battlefield: true,
                power_toughness: None,
                keywords: Vec::new(),
                triggered_abilities: Vec::new(),
            },
            |definition| cards::Characteristics::from_printed(definition, true),
        )
        .with_effects(&applicable)
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::keywords;
    use crate::rules;
    use crate::testing::*;

    use super::characteristics;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(
                    !characteristics(&answers, *bears)
                        .keywords
                        .contains(&keywords::StaticKeyword::Vigilance)
                );
            })
            .unwrap();
        Ok(())
    }
}
