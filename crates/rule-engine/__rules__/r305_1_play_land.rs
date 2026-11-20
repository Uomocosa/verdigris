use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res};

use crate::BATTLEFIELD;
use crate::HAND;
use crate::STACK;
use crate::actions;
use crate::cards;
use crate::players;
use crate::rules::helpers;
use crate::turns;
use crate::zones;

/// 305.1 A player who has priority may play a land card from their hand during a main phase of
/// their turn when the stack is empty. Playing a land is a special action; it doesn't use the
/// stack (see rule 116). Rather, the player simply puts the land onto the battlefield. Since the
/// land doesn't go on the stack, it is never a spell, and players can't respond to it with
/// instants or activated abilities.
///
/// 305.2 A player can normally play one land during their turn; however, continuous effects may
/// increase this number.
///
/// 305.3 A player can't play a land, for any reason, if it isn't their turn. Ignore any part of
/// an effect that instructs a player to do so.
///
/// 116.2a Playing a land is a special action. To play a land, a player puts that land onto the
/// battlefield from the zone it was in (usually that player's hand). By default, a player can
/// take this action only once during each of their turns. A player can take this action any time
/// they have priority and the stack is empty during a main phase of their turn. See rule 305,
/// "Lands."
pub fn r305_1_play_land(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    turn: Res<turns::Turn>,
    lands: Query<(&cards::CardDefinition, &cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
    mut played: Query<&mut players::LandsPlayedThisTurn>,
) {
    let priority = priority.into_inner();
    let turn = turn.into_inner();
    for action in actions.read() {
        let actions::Action::PlayLand(land) = &action.action else {
            continue;
        };
        let land = *land;
        let Ok((definition, owner, in_zone)) = lands.get(land) else {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        };
        let Some(hand) = helpers::find_zone(HAND, Some(action.player), &zone_entities) else {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        };
        let Some(stack) = helpers::find_zone(STACK, None, &zone_entities) else {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        };
        let is_stack_empty = !lands.iter().any(|(_, _, zone)| **zone == stack);
        let is_main_phase = matches!(
            turn.step,
            turns::Step::PrecombatMain | turns::Step::PostcombatMain
        );
        let do_special_action_conditions_hold = action.player == priority.player
            && **owner == action.player
            && **in_zone == hand
            && definition.types.contains(&cards::CardType::Land)
            && is_stack_empty
            && is_main_phase
            && turn.active == action.player;
        if !do_special_action_conditions_hold {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        }
        let Ok(mut record) = played.get_mut(action.player) else {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        };
        if record.has_played_all_this_turn(turn.number) {
            rejected.write(actions::Rejected::CannotPlayLand { land });
            continue;
        }
        record.record_play(turn.number);
        if let Some(battlefield) =
            helpers::find_zone(BATTLEFIELD, Some(action.player), &zone_entities)
        {
            commands.entity(land).insert(zones::InZone(battlefield));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn test_usage() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        game.play([p1.play_land(forest)])?;
        assert_eq!(game.zone_of(forest), BATTLEFIELD);
        Ok(())
    }

    #[test]
    fn r305_1_a_player_plays_a_land_from_their_hand_as_a_special_action() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([p1.play_land(forest)])?;

        assert_eq!(game.zone_of(forest), BATTLEFIELD);
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn r305_2_a_second_land_in_the_same_turn_is_rejected() -> Result<(), TestError> {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST, FOREST]), P2]);
        let [first, second] = game.player(p1).cards_in(HAND).exactly::<2>(FOREST)?;

        game.play([p1.play_land(first)])?;
        game.act(p1, actions::Action::PlayLand(second.into()));

        assert_eq!(game.zone_of(first), BATTLEFIELD);
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlayLand {
                land: second.into()
            })
        );
        assert_eq!(game.zone_of(second), HAND);
        Ok(())
    }

    #[test]
    fn r305_3_a_land_cannot_be_played_during_an_opponents_turn() -> Result<(), TestError> {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([FOREST]), P2.active()]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        game.play([p2.pass_priority()])?;

        game.act(p1, actions::Action::PlayLand(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlayLand {
                land: forest.into()
            })
        );
        assert_eq!(game.zone_of(forest), HAND);
        Ok(())
    }
}
