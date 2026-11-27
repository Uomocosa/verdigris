use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Res};

use crate::Attempt;
use crate::BATTLEFIELD;
use crate::actions;
use crate::cards;
use crate::events;
use crate::players;
use crate::rules;
use crate::turns;

/// 305.1. A player who has priority may play a land card from their hand during a main phase of
/// their turn when the stack is empty. Playing a land is a special action; it doesn’t use the stack
/// (see rule 116). Rather, the player simply puts the land onto the battlefield. Since the land
/// doesn’t go on the stack, it is never a spell, and players can’t respond to it with instants or
/// activated abilities.
///
/// 305.2. A player can normally play one land during their turn; however, continuous effects may
/// increase this number.
///
/// 305.3. A player can’t play a land, for any reason, if it isn’t their turn. Ignore any part of an
/// effect that instructs a player to do so.
///
/// # See also
///
/// 116.2a Playing a land is a special action. To play a land, a player puts that land onto the
/// battlefield from the zone it was in (usually that player’s hand). By default, a player can take
/// this action only once during each of their turns. A player can take this action any time they
/// have priority and the stack is empty during a main phase of their turn. See rule 305, “Lands.”
pub fn r305_1_r305_2_r305_3_play_land() -> ScheduleConfigs<ScheduleSystem> {
    (propose_play, check_play).chain()
}

fn propose_play(mut actions: MessageReader<actions::PlayerAction>, mut commands: Commands) {
    for action in actions.read() {
        let actions::Action::Play(card) = &action.action else {
            continue;
        };
        commands.entity(*card).insert(cards::Controller(action.player));
    }
}

fn check_play(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut entering: MessageWriter<Attempt<events::EnterBattlefield>>,
    mut commands: Commands,
    state: Res<turns::TurnState>,
    answers: rules::Rules,
) {
    let state = state.into_inner();
    for action in actions.read() {
        let actions::Action::Play(card) = &action.action else {
            continue;
        };
        let card = *card;
        if !answers.legal::<events::Play>(&events::Play { card }) {
            commands.entity(card).remove::<cards::Controller>();
            rejected.write(actions::Rejected::CannotPlay { card });
            continue;
        }
        let Ok(record) = answers
            .entities
            .get(*action.player)
            .map(|entity| entity.get::<players::LandsPlayedThisTurn>().copied())
        else {
            commands.entity(card).remove::<cards::Controller>();
            rejected.write(actions::Rejected::CannotPlay { card });
            continue;
        };
        let Some(mut record) = record else {
            commands.entity(card).remove::<cards::Controller>();
            rejected.write(actions::Rejected::CannotPlay { card });
            continue;
        };
        if !answers
            .value::<rules::LandPlays>(*action.player)
            .allows(u32::from(record.count_for(state.turn.number)))
        {
            commands.entity(card).remove::<cards::Controller>();
            rejected.write(actions::Rejected::CannotPlay { card });
            continue;
        }
        record.record_play(state.turn.number);
        commands.entity(*action.player).insert(record);
        if answers.zones.find(BATTLEFIELD, Some(action.player)).is_some() {
            entering.write(Attempt(events::EnterBattlefield::from_controller(
                action.player,
                card,
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn a_player_plays_a_land_from_their_hand_as_a_special_action() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([p1.play_land(forest)])?;

        assert!(game.moved(forest).to(BATTLEFIELD).of(p1));
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn a_second_land_in_the_same_turn_is_rejected() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST, FOREST]), P2]);
        let [forest_1, forest_2] = game.player(p1).cards_in(HAND).exactly::<2>(FOREST)?;

        game.play([p1.play_land(forest_1)])?;
        game.act(p1, actions::Action::Play(forest_2.into()));

        assert!(game.moved(forest_1).to(BATTLEFIELD).of(p1));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest_2.into()
            })
        );
        assert!(game.card(forest_2).is_in(HAND).of(p1));
        Ok(())
    }

    #[test]
    fn azusa_lost_but_seeking_allows_two_additional_lands() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.battlefield([AZUSA_LOST_BUT_SEEKING]).hand([FOREST, FOREST, FOREST, FOREST]),
            P2,
        ]);
        let [forest_1, forest_2, forest_3, forest_4] =
            game.player(p1).cards_in(HAND).exactly::<4>(FOREST)?;

        game.play([p1.play_land(forest_1)])?;
        game.play([p1.play_land(forest_2)])?;
        game.play([p1.play_land(forest_3)])?;
        game.act(p1, actions::Action::Play(forest_4.into()));

        assert!(game.moved(forest_1).to(BATTLEFIELD).of(p1));
        assert!(game.moved(forest_2).to(BATTLEFIELD).of(p1));
        assert!(game.moved(forest_3).to(BATTLEFIELD).of(p1));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest_4.into()
            })
        );
        assert!(game.card(forest_4).is_in(HAND).of(p1));
        Ok(())
    }

    #[test]
    fn a_land_cannot_be_played_during_an_opponents_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([FOREST]), P2.active()]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        game.play([p2.pass_priority()])?;

        game.act(p1, actions::Action::Play(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest.into()
            })
        );
        assert!(game.card(forest).is_in(HAND).of(p1));
        Ok(())
    }

    #[test]
    fn crucible_of_worlds_lets_a_player_play_a_land_from_their_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]),
            P2,
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [_forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])?;
        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST)?;
        let [mind_rot_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(MIND_ROT)?;
        game.play([p1.arrange_graveyard([forest_buried, mind_rot_buried])])?;
        game.play([p1.play_land(forest_buried)])?;

        assert!(game.moved(forest_buried).to(BATTLEFIELD).of(p1));
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn a_land_in_the_graveyard_cannot_be_played_without_crucible_of_worlds() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]), P2]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));

        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST)?;
        game.act(p1, actions::Action::Play(forest_buried.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest_buried.into()
            })
        );
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));
        Ok(())
    }

    #[test]
    fn crucible_of_worlds_does_not_let_a_player_play_a_land_from_an_opponents_graveyard()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([FOREST]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [forest] = game.player(p2).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(mind_rot).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.moved(forest).to(GRAVEYARD).of(p2));

        let [forest_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(FOREST)?;
        game.act(p1, actions::Action::Play(forest_buried.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest_buried.into()
            })
        );
        assert!(game.moved(forest).to(GRAVEYARD).of(p2));
        Ok(())
    }

    #[test]
    fn crucible_of_worlds_does_not_let_a_player_play_a_land_on_an_opponents_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CRUCIBLE_OF_WORLDS]).hand([MIND_ROT, FOREST]).mana_pool(mana![2, B]),
            P2,
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(mind_rot).targeting(p1).pass_priority(),
            p2.pass_priority(),
        ])?;
        let [forest_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(FOREST)?;
        let [mind_rot_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(MIND_ROT)?;
        game.play([p1.arrange_graveyard([forest_buried, mind_rot_buried])])?;
        game.play([p1.end_turn(), p2.end_turn()])?;
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));

        game.act(p1, actions::Action::Play(forest_buried.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest_buried.into()
            })
        );
        assert!(game.moved(forest).to(GRAVEYARD).of(p1));
        Ok(())
    }
}
