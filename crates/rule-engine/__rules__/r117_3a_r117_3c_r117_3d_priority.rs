use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::GameMut;
use crate::actions;
use crate::turns;

/// 117.3a The active player receives priority at the beginning of most steps and phases, after any
/// turn-based actions (such as drawing a card during the draw step; see rule 703) have been dealt
/// with and abilities that trigger at the beginning of that phase or step have been put on the
/// stack. No player receives priority during the untap step. Players usually don’t get priority
/// during the cleanup step (see rule 514.3).
///
/// 117.3c If a player has priority when they cast a spell, activate an ability, or take a special
/// action, that player receives priority afterward.
///
/// 117.3d If a player has priority and chooses not to take any actions, that player passes. If any
/// mana is in that player’s mana pool, they announce what mana is there. Then the next player in
/// turn order receives priority.
pub fn r117_3a_r117_3c_r117_3d_priority(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut game: GameMut,
    turn_order: Res<turns::TurnOrder>,
    window: Res<actions::OpeningHandWindow>,
) {
    let turn_order = turn_order.into_inner();
    let window = window.into_inner();
    for action in actions.read() {
        if matches!(&action.action, actions::Action::PassPriority)
            && !window.is_complete(turn_order.len())
        {
            rejected.write(actions::Rejected::NoPriority {
                player: action.player,
            });
            continue;
        }
        if action.player != game.priority_holder().id {
            if matches!(&action.action, actions::Action::PassPriority) {
                rejected.write(actions::Rejected::NoPriority {
                    player: action.player,
                });
            }
            continue;
        }
        if matches!(&action.action, actions::Action::PassPriority) {
            if let Some(next) = game.priority_holder().next_in_turn_order() {
                game.pass_priority_to(next.id);
            }
            continue;
        }
        game.give_priority_to(action.player);
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn passing_moves_priority_to_the_next_player_in_turn_order() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.pass_priority()])?;

        let priority = game.priority();
        assert_eq!(priority.holder, p2.id);
        assert!(priority.has_passed(p1.id));
        assert_ne!(priority.holder, p1.id);
        Ok(())
    }

    #[test]
    fn a_four_player_game_passes_priority_in_a_ring() -> TestResult {
        let (mut game, [p1, p2, p3, p4]) = MockGame::new([P1, P2, P3, P4]);
        let expected = [p2, p3, p4];
        let holders = [p1, p2, p3];

        for (holder, next) in holders.into_iter().zip(expected) {
            game.play([holder.pass_priority()])?;
            assert_eq!(game.priority_player(), next.id);
        }
        Ok(())
    }

    #[test]
    fn the_caster_keeps_priority_after_casting_a_spell() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1, P2.hand([LIGHTNING_BOLT]).mana_pool(mana![R])]);
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;

        game.play([p1.pass_priority(), p2.cast(bolt)])?;

        let priority = game.priority();
        assert_eq!(priority.holder, p2.id);
        assert!(priority.passed.is_empty());
        Ok(())
    }

    #[test]
    fn a_player_without_priority_cannot_pass() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        let err = game.play([p2.pass_priority()]).unwrap_err();

        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::NoPriority { player: p2.id })
        );
        assert_eq!(game.priority_player(), p1.id);
    }

    #[test]
    fn a_player_who_left_does_not_receive_priority() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.act(p2, actions::Action::Concede);
        game.play([p1.pass_priority()])?;

        assert_eq!(game.priority_player(), p3.id);
        Ok(())
    }
}
