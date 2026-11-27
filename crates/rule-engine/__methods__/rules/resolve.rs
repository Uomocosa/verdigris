use crate::cards;
use crate::players;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;
use crate::stack;

pub fn resolve<E: EventArgs>(
    answers: &rules::Rules,
    reference: rules::ArgRef,
    event: &E,
) -> Option<rules::ArgValue> {
    let id = match reference {
        rules::ArgRef::Arg(id) => return event.arg(id),
        rules::ArgRef::ControllerOf(id)
        | rules::ArgRef::OwnerOf(id)
        | rules::ArgRef::SourceOf(id) => id,
    };
    let entity = match event.arg(id)? {
        rules::ArgValue::Entity(entity) => entity,
        rules::ArgValue::Player(player) => return Some(rules::ArgValue::Player(player)),
        rules::ArgValue::Entities(elements) => {
            let mut holders = elements.iter().filter_map(|element| {
                let found = answers.entities.get(*element).ok()?;
                if found.get::<players::Player>().is_some() {
                    Some(*element)
                } else {
                    match reference {
                        rules::ArgRef::ControllerOf(_) => found
                            .get::<cards::Controller>()
                            .map(|controller| ***controller)
                            .or_else(|| found.get::<cards::Owner>().map(|owner| ***owner)),
                        rules::ArgRef::OwnerOf(_) => {
                            found.get::<cards::Owner>().map(|owner| ***owner)
                        }
                        rules::ArgRef::Arg(_) | rules::ArgRef::SourceOf(_) => None,
                    }
                }
            });
            let first = holders.next()?;
            return holders
                .all(|holder| holder == first)
                .then_some(rules::ArgValue::Player(players::PlayerId(first)));
        }
        rules::ArgValue::Amount(_) | rules::ArgValue::Event(_) => return None,
    };
    let found = answers.entities.get(entity).ok()?;
    if found.get::<players::Player>().is_some() {
        return Some(rules::ArgValue::Player(players::PlayerId(entity)));
    }
    if let rules::ArgRef::SourceOf(_) = reference {
        return Some(rules::ArgValue::Entity(
            found.get::<stack::TriggeredAbilityOnStack>().map_or(entity, |ability| ability.source),
        ));
    }
    match reference {
        rules::ArgRef::ControllerOf(_) => found
            .get::<cards::Controller>()
            .map(|controller| rules::ArgValue::Player(**controller))
            .or_else(|| found.get::<cards::Owner>().map(|owner| rules::ArgValue::Player(**owner))),
        rules::ArgRef::OwnerOf(_) => {
            found.get::<cards::Owner>().map(|owner| rules::ArgValue::Player(**owner))
        }
        rules::ArgRef::Arg(_) | rules::ArgRef::SourceOf(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::resolve;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let damage = events::DealDamage {
            source: *bears,
            recipient: *p1.id,
            amount: 2,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    resolve(&answers, events::DealDamage::SOURCE.controller(), &damage),
                    Some(rules::ArgValue::Player(p1.id))
                );
                assert_eq!(
                    resolve(
                        &answers,
                        rules::ArgRef::Arg(events::DealDamage::AMOUNT),
                        &damage
                    ),
                    Some(rules::ArgValue::Amount(2))
                );
            })
            .unwrap();
        Ok(())
    }
}
