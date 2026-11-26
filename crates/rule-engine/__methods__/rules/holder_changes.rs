use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

pub fn holder_changes<E: Event>(
    answers: &rules::Rules,
    event: &E,
    target: rules::Target,
) -> Vec<rules::Change> {
    let mut visited: Vec<(rules::ArgRef, Entity)> = Vec::new();
    let mut kept = Vec::new();
    for id in E::args() {
        for reference in [
            rules::ArgRef::Arg(*id),
            rules::ArgRef::ControllerOf(*id),
            rules::ArgRef::OwnerOf(*id),
        ] {
            let Some(entity) = answers.resolve(reference, event).and_then(|value| match value {
                rules::ArgValue::Entity(entity) => Some(entity),
                rules::ArgValue::Player(player) => Some(*player),
                rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_) => None,
            }) else {
                continue;
            };
            if visited.contains(&(reference, entity)) {
                continue;
            }
            visited.push((reference, entity));
            for change in answers.changes(entity, target) {
                let held = match &change {
                    rules::Change::Restrict { holder, .. }
                    | rules::Change::AsThough { holder }
                    | rules::Change::Allow { holder, .. } => *holder == reference,
                    _ => false,
                };
                if held {
                    kept.push(change);
                }
            }
        }
    }
    kept
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::holder_changes;

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
                assert_eq!(
                    holder_changes(
                        &answers,
                        &waived_event,
                        rules::Target::Restriction(UNTAPPED)
                    ),
                    vec![rules::Change::AsThough {
                        holder: TestRule::ATTACKER.into(),
                    }]
                );
                assert_eq!(
                    holder_changes(&answers, &plain_event, rules::Target::Restriction(UNTAPPED)),
                    Vec::new()
                );
            })
            .unwrap();
        Ok(())
    }
}
