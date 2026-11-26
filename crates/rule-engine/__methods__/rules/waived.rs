use crate::rules;
use crate::rules::traits::event::Event;

pub fn waived<E: Event>(
    answers: &rules::Rules,
    event: &E,
    target: rules::Target,
    unwaivable: bool,
) -> bool {
    if unwaivable {
        return false;
    }
    let subject = event.subject();
    let other = event.other();
    let player = event.player();
    [
        (rules::Role::Subject, Some(subject)),
        (rules::Role::Other, other),
        (rules::Role::Player, Some(*player)),
    ]
    .into_iter()
    .filter_map(|(role, entity)| entity.map(|entity| (role, entity)))
    .any(|(role, entity)| {
        answers.changes(entity, target).iter().any(|change| match change {
            rules::Change::AsThough { holder } => *holder == role,
            rules::Change::Allow { holder, when } => {
                *holder == role && answers.eval(&when.bind(event), subject, other, Some(*player))
            }
            _ => false,
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::waived;

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
                let waived_event = TestRule {
                    attacker: *waiver,
                    player: p1.id,
                };
                let plain_event = TestRule {
                    attacker: *bears,
                    player: p1.id,
                };
                assert!(waived(
                    &answers,
                    &waived_event,
                    rules::Target::Restriction(UNTAPPED),
                    false
                ));
                assert!(!waived(
                    &answers,
                    &waived_event,
                    rules::Target::Restriction(UNTAPPED),
                    true
                ));
                assert!(!waived(
                    &answers,
                    &plain_event,
                    rules::Target::Restriction(UNTAPPED),
                    false
                ));
            })
            .unwrap();
        Ok(())
    }
}
