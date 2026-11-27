use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::zones;

#[must_use]
pub fn solve<E: Event>(
    answers: &rules::Rules,
    fixed: &[(rules::ArgId, rules::ArgValue)],
    viewer: players::PlayerId,
) -> Vec<(E, rules::Legality)> {
    if E::kinds().is_empty() {
        return Vec::new();
    }
    let mut domains: Vec<Vec<rules::ArgValue>> = Vec::new();
    for (index, id) in E::args().iter().enumerate() {
        if let Some(fixed) = fixed.iter().find(|(fixed_id, _)| fixed_id == id) {
            domains.push(vec![fixed.1.clone()]);
            continue;
        }
        match E::kinds().get(index) {
            Some(rules::ArgKind::Player) => domains.push(
                answers
                    .players_in_game(rules::Players::All, Some(viewer))
                    .into_iter()
                    .map(rules::ArgValue::Player)
                    .collect(),
            ),
            Some(rules::ArgKind::Entity) => {
                let mut entities: Vec<Entity> = answers
                    .entities
                    .iter()
                    .filter_map(|entity| {
                        let in_zone = entity.get::<zones::InZone>()?;
                        let (_, zone, owner) = answers.zones.get(**in_zone).ok()?;
                        if matches!(
                            *zone,
                            zones::Zone::Battlefield
                                | zones::Zone::Graveyard
                                | zones::Zone::Exile
                                | zones::Zone::Stack
                        ) || *zone == zones::Zone::Hand
                            && owner.is_some_and(|owner| ***owner == *viewer)
                        {
                            Some(entity.id())
                        } else {
                            None
                        }
                    })
                    .collect();
                entities.sort();
                domains.push(entities.into_iter().map(rules::ArgValue::Entity).collect());
            }
            Some(rules::ArgKind::Entities | rules::ArgKind::Amount | rules::ArgKind::Event)
            | None => return Vec::new(),
        }
    }
    let mut combinations: Vec<Vec<rules::ArgValue>> = vec![Vec::new()];
    for domain in &domains {
        let mut extended: Vec<Vec<rules::ArgValue>> = Vec::new();
        for prefix in &combinations {
            for value in domain {
                let mut combination = prefix.clone();
                combination.push(value.clone());
                extended.push(combination);
            }
        }
        combinations = extended;
    }
    let mut solved: Vec<(E, rules::Legality)> = Vec::new();
    for combination in &combinations {
        if let Some(event) = E::from_args(combination) {
            let legality = answers.legality(&event);
            solved.push((event, legality));
        }
    }
    solved
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::solve;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([FOREST, GRIZZLY_BEARS]), P2.hand([FOREST])]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [stranger] = game.player(p2).cards_in(HAND).exactly::<1>(FOREST)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let solved = solve::<events::Play>(
                    &answers,
                    &[(events::Play::PLAYER, rules::ArgValue::Player(p1.id))],
                    p1.id,
                );
                assert!(solved.iter().any(|(event, legality)| event.card == *forest
                    && *legality == rules::Legality::Legal));
                assert!(solved.iter().any(|(event, legality)| event.card == *bears
                    && *legality
                        == rules::Legality::Illegal(vec![rules::Why::Restriction(
                            events::play::LAND_CARD
                        )])));
                assert!(
                    solved.iter().all(|(event, _)| event.card == *forest || event.card == *bears)
                );
                assert!(solved.iter().all(|(event, _)| event.card != *stranger));
            })
            .unwrap();
        Ok(())
    }
}
