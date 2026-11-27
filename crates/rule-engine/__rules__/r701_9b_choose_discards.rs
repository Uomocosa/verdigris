use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Entity, MessageReader, MessageWriter};

use crate::Attempt;
use crate::Resolve;
use crate::actions;
use crate::events;
use crate::players;

/// 701.9b By default, effects that cause a player to discard a card allow the affected player to
/// choose which card to discard. Some effects, however, require a random discard or allow another
/// player to choose which card is discarded.
pub fn r701_9b_choose_discards() -> ScheduleConfigs<ScheduleSystem> {
    (ask_which_cards, discard_chosen_cards).chain()
}

fn ask_which_cards(
    mut discards: MessageReader<Resolve<events::Discard>>,
    mut asks: MessageWriter<actions::Ask>,
    mut discards_card: MessageWriter<Attempt<events::DiscardCard>>,
    players: players::Players,
) {
    for discard in discards.read() {
        let Some(affected) = players.affected_by(&**discard).into_iter().next() else {
            continue;
        };
        let hand: Vec<Entity> = affected.hand().into_iter().map(|card| *card).collect();
        if hand.len() <= usize::try_from(discard.amount).unwrap_or(usize::MAX) {
            for card in hand {
                discards_card.write(Attempt(events::DiscardCard {
                    card,
                    player: discard.player,
                }));
            }
            continue;
        }
        asks.write(actions::Ask(actions::Choice::Discard(
            actions::DiscardChoice {
                player: discard.player,
                count: discard.amount,
                legal: hand.into_iter().collect(),
            },
        )));
    }
}

fn discard_chosen_cards(
    mut answers: MessageReader<actions::Answered>,
    mut discards_card: MessageWriter<Attempt<events::DiscardCard>>,
) {
    for answered in answers.read() {
        let actions::Choice::Discard(choice) = &answered.choice else {
            continue;
        };
        let actions::Action::ChooseDiscard(chosen) = &answered.answer else {
            continue;
        };
        for card in chosen {
            discards_card.write(Attempt(events::DiscardCard {
                card: *card,
                player: choice.player,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn mind_rot_makes_the_target_choose_two_cards_to_discard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([GRIZZLY_BEARS, LIGHTNING_BOLT, MURDER]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;
        assert_eq!(game.priority().holder, None);
        assert_eq!(game.turn().step, turns::Step::PrecombatMain);
        assert_eq!(game.turn().number, 1);

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
