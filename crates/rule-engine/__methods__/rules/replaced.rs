use crate::rules;
use crate::rules::traits::event::Event;

pub fn replaced<E: Event>(
    answers: &rules::Rules,
    result: rules::ResultId,
    event: &E,
) -> Option<rules::Then> {
    answers.holder_changes(event, rules::Target::Result(result)).into_iter().find_map(|change| {
        match change {
            rules::Change::Instead { when, then, .. } => {
                answers.eval(&when.bind(event), event, None).then_some(then)
            }
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::LOSES_LIFE;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::replaced;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([PLAGUE_STINGER]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [stinger] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(PLAGUE_STINGER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let infecting = events::DealDamage {
                    source: *stinger,
                    recipient: *bears,
                    amount: 1,
                };
                assert!(replaced(&answers, LOSES_LIFE, &infecting,).is_some());
                let plain = events::DealDamage {
                    source: *bears,
                    recipient: *stinger,
                    amount: 2,
                };
                assert_eq!(replaced(&answers, LOSES_LIFE, &plain,), None);
            })
            .unwrap();
        Ok(())
    }
}
