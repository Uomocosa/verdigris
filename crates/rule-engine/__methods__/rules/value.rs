use bevy::prelude::Entity;

use crate::effects;
use crate::rules;

pub fn value<V: rules::traits::value_rule::ValueRule>(
    answers: &rules::Rules,
    player: Entity,
) -> rules::Limit {
    let mut limit = V::default();
    for modification in answers.mods(player) {
        let effects::Modification::Rule(rules::RuleChange::Value { rule, change }) = modification
        else {
            continue;
        };
        if rule != V::ID {
            continue;
        }
        limit = match change {
            rules::ValueChange::Set(count) => rules::Limit::At(count),
            rules::ValueChange::Increase(count) => match limit {
                rules::Limit::At(maximum) => rules::Limit::At(maximum.saturating_add(count)),
                rules::Limit::NoMaximum => rules::Limit::NoMaximum,
            },
            rules::ValueChange::Decrease(count) => match limit {
                rules::Limit::At(maximum) => rules::Limit::At(maximum.saturating_sub(count)),
                rules::Limit::NoMaximum => rules::Limit::NoMaximum,
            },
            rules::ValueChange::NoMaximum => rules::Limit::NoMaximum,
        };
    }
    limit
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::rules;
    use crate::testing::*;

    use super::value;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    value::<rules::MaximumHandSize>(&answers, p1.into()),
                    rules::Limit::At(7)
                );
            })
            .unwrap();
    }

    #[test]
    fn fold_applies_increase_and_no_maximum_over_defaults() {
        let (mut game, [p1, _]) = MockGame::new([
            P1.battlefield([AZUSA_LOST_BUT_SEEKING, RELIQUARY_TOWER]),
            P2,
        ]);
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    value::<rules::LandPlays>(&answers, p1.into()),
                    rules::Limit::At(3)
                );
                assert_eq!(
                    value::<rules::MaximumHandSize>(&answers, p1.into()),
                    rules::Limit::NoMaximum
                );
            })
            .unwrap();
    }
}
