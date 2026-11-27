use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Entity, MessageReader, MessageWriter};

use crate::Attempt;
use crate::HAND;
use crate::actions;
use crate::effects;
use crate::events;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

/// 701.9b By default, effects that cause a player to discard a card allow the affected player to
/// choose which card to discard. Some effects, however, require a random discard or allow another
/// player to choose which card is discarded.
pub fn r701_9b_choose_discards() -> ScheduleConfigs<ScheduleSystem> {
    (ask_which_cards, discard_chosen_cards).chain()
}

fn ask_which_cards(
    mut requests: MessageReader<actions::DiscardRequest>,
    mut asks: MessageWriter<actions::Ask>,
    mut discards: MessageWriter<Attempt<events::Discard>>,
    players: players::Players,
    answers: rules::Rules,
) {
    for request in requests.read() {
        let Some(affected) =
            players.still_in_the_game().into_iter().find(|player| player.id == request.player)
        else {
            continue;
        };
        let hand: Vec<Entity> = affected.hand().into_iter().map(|card| *card).collect();
        if !answers.legal(&events::Discard {
            cards: hand.clone(),
            cause: request.cause,
        }) {
            continue;
        }
        if hand.len() <= usize::try_from(request.count).unwrap_or(usize::MAX) {
            discards.write(Attempt(events::Discard {
                cards: hand,
                cause: request.cause,
            }));
            continue;
        }
        asks.write(actions::Ask(actions::Choice::Pick(actions::PickChoice {
            id: events::discard::DISCARD,
            player: request.player,
            source: None,
            among: actions::Candidates::Matching {
                filter: effects::Filter::from_in_zone(HAND, rules::YOU),
                context: events::AbilityContext {
                    spell_or_ability: *request.player,
                },
                check: actions::CandidateCheck::Discard {
                    cause: request.cause,
                },
            },
            count: actions::Count::exactly(request.count),
            ordered: false,
        })));
    }
}

fn discard_chosen_cards(
    mut answers: MessageReader<actions::Answered>,
    mut discards: MessageWriter<Attempt<events::Discard>>,
) {
    for answered in answers.read() {
        let actions::Choice::Pick(choice) = &answered.choice else {
            continue;
        };
        if choice.id != events::discard::DISCARD {
            continue;
        }
        let actions::Action::Choose(chosen) = &answered.answer else {
            continue;
        };
        discards.write(Attempt(events::Discard {
            cards: chosen.clone(),
            cause: events::Discard::ID,
        }));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn mind_rot_makes_the_target_choose_two_cards_to_discard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.battlefield([FOREST]).hand([GRIZZLY_BEARS, LIGHTNING_BOLT, MURDER]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [murder] = game.player(p2).cards_in(HAND).exactly::<1>(MURDER)?;
        let [forest] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;
        assert_eq!(game.priority().holder, None);
        assert_eq!(game.turn().step, turns::Step::PrecombatMain);
        assert_eq!(game.turn().number, 1);

        let actions::Choice::Pick(choice) = game
            .app
            .world()
            .get_resource::<actions::PendingChoice>()
            .and_then(|pending| (**pending).clone())
            .expect("a pending discard choice")
        else {
            panic!("expected a pick choice");
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = choice.candidates(&answers);
                let legal: BTreeSet<Entity> = listed
                    .iter()
                    .filter(|(_, legality)| *legality == rules::Legality::Legal)
                    .map(|(entity, _)| *entity)
                    .collect();
                assert_eq!(legal, BTreeSet::from([*bears, *bolt, *murder]));
                assert!(listed.iter().any(|(entity, legality)| *entity == *forest
                    && matches!(
                        legality,
                        rules::Legality::Illegal(reasons)
                        if reasons.contains(&rules::Why::NotMatching)
                    )));
            })
            .unwrap();

        game.play([p2.discard([bears, bolt])])?;
        let [bears_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p2.arrange_graveyard([bolt_buried, bears_buried])])?;

        assert!(game.moved(bears).to(GRAVEYARD).of(p2));
        assert!(game.moved(bolt).to(GRAVEYARD).of(p2));
        assert!(game.player(p2).cards_in(HAND).has(MURDER));
        Ok(())
    }

    #[test]
    fn mind_rot_discards_nothing_when_the_target_cant_discard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.battlefield([TEST_CANT_DISCARD]).hand([GRIZZLY_BEARS, LIGHTNING_BOLT, MURDER]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;

        assert_eq!(game.player(p2).cards_in(HAND).count(), 3);
        assert!(game.player(p2).cards_in(HAND).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p2).cards_in(HAND).has(MURDER));
        assert_eq!(game.player(p2).cards_in(GRAVEYARD).count(), 0);
        Ok(())
    }
}
