use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, With};

use crate::DEFENDING_PLAYER;
use crate::Game;
use crate::actions;
use crate::players;
use crate::turns;

/// 507.1. First, if the game being played is a multiplayer game in which the active player’s
/// opponents don’t all automatically become defending players, the active player chooses one of
/// their opponents. That player becomes the defending player. This turn-based action doesn’t use
/// the stack. (See rule 506.2.)
///
/// # See also
///
/// 506.2a During the combat phase of a multiplayer game, there may be one or more defending
/// players, depending on the variant being played and the options chosen for it. Unless all the
/// attacking player’s opponents automatically become defending players during the combat phase,
/// the attacking player chooses one of their opponents as a turn-based action during the
/// beginning of combat step. (Note that the choice may be dictated by the variant being played or
/// the options chosen for it.) That player becomes the defending player. See rule 802, “Attack
/// Multiple Players Option,” rule 803, “Attack Left and Attack Right Options,” and rule 809,
/// “Emperor Variant.”
pub fn r507_1_choose_defending_player() -> ScheduleConfigs<ScheduleSystem> {
    (
        ask_which_opponent.run_if(turns::Step::BeginningOfCombat.begins()),
        mark_the_chosen_opponent,
    )
        .chain()
}

fn ask_which_opponent(mut asks: MessageWriter<actions::Ask>, game: Game) {
    let Some(attacking_player) = game.active_player() else {
        return;
    };
    let opponents: Vec<Entity> = attacking_player
        .opponents_still_in_the_game()
        .into_iter()
        .map(|opponent| *opponent.id)
        .collect();
    if opponents.len() < 2 {
        return;
    }
    asks.write(actions::Ask(actions::Choice::Pick(actions::PickChoice {
        id: DEFENDING_PLAYER,
        player: attacking_player.id,
        source: None,
        among: actions::Candidates::Fixed(opponents.into_iter().collect()),
        count: actions::Count::exactly(1),
        ordered: false,
    })));
}

fn mark_the_chosen_opponent(
    mut answers: MessageReader<actions::Answered>,
    mut commands: Commands,
    defending: Query<Entity, With<players::DefendingPlayer>>,
    players: players::Players,
) {
    for answered in answers.read() {
        let actions::Choice::Pick(choice) = &answered.choice else {
            continue;
        };
        if choice.id != DEFENDING_PLAYER {
            continue;
        }
        let actions::Action::Choose(chosen) = &answered.answer else {
            continue;
        };
        let [chosen] = chosen.as_slice() else {
            continue;
        };
        let chosen = players::PlayerId(*chosen);
        if !players.is_still_in_the_game(chosen) {
            continue;
        }
        for player in defending.iter() {
            commands.entity(player).remove::<players::DefendingPlayer>();
        }
        commands.entity(*chosen).insert(players::DefendingPlayer);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::players;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn the_active_player_chooses_one_opponent_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1,
                P2.active().battlefield([GRIZZLY_BEARS]),
                P3.battlefield([GRIZZLY_BEARS]),
            ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p2.pass_to(turns::Step::BeginningOfCombat),
            p1.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
            p2.choose_defending_player(p3),
        ])?;
        game.play([
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.pass_to(turns::Step::DeclareAttackers),
            p3.pass_to(turns::Step::DeclareAttackers),
            p2.declare_attackers([bears]),
        ])?;
        game.play([
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_to(turns::Step::CombatDamage),
            p3.pass_to(turns::Step::CombatDamage),
        ])?;
        game.play([p2.pass_priority(), p3.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.life(p3), 18);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }

    #[test]
    fn a_player_who_has_lost_cannot_be_chosen_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3, p4]) = MockGame::new([P1, P2, P3, P4.life(0)]);
        game.act(p1, actions::Action::Choose(vec![p4.into()]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NothingToChoose { player: p1.id })
        );
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
            p4.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        assert!(game.has_lost(p4));

        game.act(p1, actions::Action::Choose(vec![p4.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidChoice {
                player: p1.id,
                why: actions::ChoiceWhy::NotAllowed(p4.into()),
            })
        );
        assert!(!game.has::<players::DefendingPlayer>(p4));
        Ok(())
    }
}
