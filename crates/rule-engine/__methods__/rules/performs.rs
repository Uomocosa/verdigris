use crate::rules;
use crate::rules::traits::event::Event;

pub fn performs<E: Event>(answers: &rules::Rules, event: &E) -> bool {
    let mut performs = true;
    for id in E::args() {
        let Some(entity) = event.arg(*id).and_then(|value| match value {
            rules::ArgValue::Entity(entity) => Some(entity),
            rules::ArgValue::Player(player) => Some(*player),
            _ => None,
        }) else {
            continue;
        };
        for change in answers.changes(entity, rules::Target::event::<E>()) {
            match change {
                rules::Change::Skip { when } if answers.eval(&when.bind(event), event, None) => {
                    performs = false;
                }
                rules::Change::Perform { when } if answers.eval(&when.bind(event), event, None) => {
                    performs = true;
                }
                _ => {}
            }
        }
    }
    performs
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::performs;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([TEST_RULE_WAIVER, GRIZZLY_BEARS]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!performs(
                    &answers,
                    &TestStep {
                        permanent: *waiver,
                        controller: p1.id,
                    }
                ));
                assert!(performs(
                    &answers,
                    &TestStep {
                        permanent: *bears,
                        controller: p1.id,
                    }
                ));
            })
            .unwrap();
        Ok(())
    }
}
