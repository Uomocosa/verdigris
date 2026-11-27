use bevy::prelude::Entity;

use crate::counters;
use crate::players;
use crate::rules;
use crate::zones;

pub fn eval_player_value(
    answers: &rules::Rules,
    quantity: rules::PlayerQuantity,
    comparison: rules::Comparison,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers
            .entities
            .get(subject)
            .ok()
            .and_then(|entity| {
                entity.get::<players::Player>()?;
                match quantity {
                    rules::PlayerQuantity::Life => {
                        entity.get::<players::Life>().map(|life| comparison.holds(**life))
                    }
                    rules::PlayerQuantity::Counters(kind) => {
                        let counters =
                            entity.get::<counters::Counters>().map_or(0, |held| held.get(kind));
                        i32::try_from(counters).ok().map(|value| comparison.holds(value))
                    }
                    rules::PlayerQuantity::CardsInHand | rules::PlayerQuantity::CardsInLibrary => {
                        let wanted = if quantity == rules::PlayerQuantity::CardsInHand {
                            zones::Zone::Hand
                        } else {
                            zones::Zone::Library
                        };
                        let found = answers
                            .in_zone
                            .iter()
                            .filter(|in_zone| {
                                answers.zones.get(***in_zone).is_ok_and(|(_, zone, owner)| {
                                    *zone == wanted
                                        && owner.is_some_and(|owner| ***owner == subject)
                                })
                            })
                            .count();
                        i32::try_from(found).ok().map(|value| comparison.holds(value))
                    }
                }
            })
            .is_some_and(|holds| holds)
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::eval_player_value;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_player_value(
                    &answers,
                    rules::PlayerQuantity::Life,
                    rules::Comparison::OrMore(20),
                    Some(*p1.id),
                ));
                assert!(!eval_player_value(
                    &answers,
                    rules::PlayerQuantity::Life,
                    rules::Comparison::OrMore(21),
                    Some(*p1.id),
                ));
                assert!(eval_player_value(
                    &answers,
                    rules::PlayerQuantity::CardsInLibrary,
                    rules::Comparison::OrMore(12),
                    Some(*p1.id),
                ));
                assert!(!eval_player_value(
                    &answers,
                    rules::PlayerQuantity::CardsInHand,
                    rules::Comparison::OrMore(1),
                    Some(*p1.id),
                ));
                assert!(!eval_player_value(
                    &answers,
                    rules::PlayerQuantity::Life,
                    rules::Comparison::OrMore(1),
                    Some(*bears),
                ));
                assert!(!eval_player_value(
                    &answers,
                    rules::PlayerQuantity::Life,
                    rules::Comparison::OrMore(1),
                    None,
                ));
            })
            .unwrap();
        Ok(())
    }
}
