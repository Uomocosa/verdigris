use bevy::prelude::Entity;

use crate::rules;

pub fn value<V: rules::traits::value_rule::ValueRule>(
    answers: &rules::Rules,
    entity: Entity,
) -> rules::Limit {
    let mut limit = V::default();
    for change in answers.changes(entity, rules::Target::Value(V::ID)) {
        limit = match change {
            rules::Change::Set(count) => rules::Limit::At(count),
            rules::Change::Increase(count) => match limit {
                rules::Limit::At(maximum) => rules::Limit::At(maximum.saturating_add(count)),
                rules::Limit::NoMaximum => rules::Limit::NoMaximum,
            },
            rules::Change::Decrease(count) => match limit {
                rules::Limit::At(maximum) => rules::Limit::At(maximum.saturating_sub(count)),
                rules::Limit::NoMaximum => rules::Limit::NoMaximum,
            },
            rules::Change::NoMaximum => rules::Limit::NoMaximum,
            _ => limit,
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
                    value::<rules::MaximumHandSize>(&answers, *p1.id),
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
                    value::<rules::LandPlays>(&answers, *p1.id),
                    rules::Limit::At(3)
                );
                assert_eq!(
                    value::<rules::MaximumHandSize>(&answers, *p1.id),
                    rules::Limit::NoMaximum
                );
            })
            .unwrap();
    }
}
