use crate::cards;
use crate::damage;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn defines<E: Event>(
    answers: &rules::Rules,
    definition: &rules::Definition,
    event: &E,
) -> bool {
    let holds = match &definition.condition {
        rules::Condition::Of(reference, inner) => match inner.as_ref() {
            rules::Condition::EventFieldAtLeastRemainingToughness(field) => {
                let viewpoint = match answers.resolve(*reference, event) {
                    Some(rules::ArgValue::Entity(entity)) => Some(entity),
                    Some(rules::ArgValue::Player(player)) => Some(*player),
                    Some(rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_)) | None => None,
                };
                viewpoint.is_some_and(|subject| {
                    let remaining = answers
                        .entities
                        .get(subject)
                        .ok()
                        .and_then(|entity| entity.get::<cards::EffectivePT>())
                        .map_or(0, |effective| {
                            u32::try_from(effective.toughness).unwrap_or_default()
                        })
                        .saturating_sub(
                            answers
                                .entities
                                .get(subject)
                                .ok()
                                .and_then(|entity| entity.get::<damage::Damage>())
                                .map_or(0, |marked| **marked),
                        );
                    event.field(*field).is_some_and(|amount| amount >= remaining)
                })
            }
            _ => answers.eval(&definition.condition.bind(event), event, None),
        },
        condition => answers.eval(&condition.bind(event), event, None),
    };
    holds
        || answers.waived(
            event,
            rules::Target::Definition(definition.id),
            definition.unwaivable,
        )
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [plain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let one_from_rats = events::AssignCombatDamage {
                    target: *bears,
                    source: *rats,
                    amount: 1,
                    player: p1.id,
                };
                let one_from_plain = events::AssignCombatDamage {
                    source: *plain,
                    ..one_from_rats
                };
                let two_from_plain = events::AssignCombatDamage {
                    amount: 2,
                    ..one_from_plain
                };
                assert!(answers.defines(&rules::Definition::lethal_damage(), &one_from_rats));
                assert!(!answers.defines(&rules::Definition::lethal_damage(), &one_from_plain));
                assert!(answers.defines(&rules::Definition::lethal_damage(), &two_from_plain));
            })
            .unwrap();
        Ok(())
    }
}
