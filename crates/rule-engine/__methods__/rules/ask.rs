use bevy::prelude::Entity;

use crate::cards;
use crate::effects;
use crate::rules;
use crate::rules::traits::rule_question::RuleQuestion;

pub fn ask<Q: RuleQuestion>(answers: &rules::Rules, entity: Entity) -> Q::Answer {
    let mut applicable: Vec<effects::ContinuousEffect> =
        answers.effects.iter().filter(|effect| effect.affected == entity).copied().collect();
    applicable.sort_by_key(|effect| effect.timestamp);
    let characteristics = answers
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
        .with_effects(&applicable);
    applicable
        .iter()
        .filter_map(|effect| {
            if let effects::Change::Rule { question, answer } = effect.change
                && question == Q::QUESTION
            {
                Q::from_answer(answer)
            } else {
                None
            }
        })
        .next_back()
        .unwrap_or_else(|| Q::default(&characteristics))
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::ask;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([FELIDAR_SOVEREIGN]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [sovereign] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(ask::<rules::TapsWhenAttacking>(&answers, *bears));
                assert!(!ask::<rules::TapsWhenAttacking>(&answers, *sovereign));
            })
            .unwrap();
        Ok(())
    }
}
