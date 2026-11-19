use bevy::prelude::{MessageReader, MessageWriter, Res, ResMut};

use crate::actions;
use crate::turns;

/// 117.3a The active player receives priority at the beginning of most steps and phases, after
/// any turn-based actions (such as drawing a card during the draw step; see rule 703) have been
/// dealt with and abilities that trigger at the beginning of that phase or step have been put on
/// the stack. No player receives priority during the untap step. Players usually don't get
/// priority during the cleanup step (see rule 514.3).
///
/// 117.3c If a player has priority when they cast a spell, activate an ability, or take a special
/// action, that player receives priority afterward.
///
/// 117.3d If a player has priority and chooses not to take any actions, that player passes. If any
/// mana is in that player's mana pool, they announce what mana is there. Then the next player in
/// turn order receives priority.
pub fn r117_3_priority(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut priority: ResMut<turns::Priority>,
    turn_order: Res<turns::TurnOrder>,
) {
    let turn_order = turn_order.into_inner();
    for action in actions.read() {
        if action.player != priority.player {
            if matches!(&action.action, actions::Action::PassPriority) {
                rejected.write(actions::Rejected::NoPriority {
                    player: action.player,
                });
            }
            continue;
        }
        if matches!(&action.action, actions::Action::PassPriority) {
            priority.consecutive_passes = priority.consecutive_passes.saturating_add(1);
            if let Some(next) = turn_order.next_after(priority.player) {
                priority.player = next;
            }
            continue;
        }
        priority.consecutive_passes = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder().build();
        let p2 = game.p2;
        game.pass_priority();
        assert_eq!(game.app.world().resource::<turns::Priority>().player, p2);
    }

    #[test]
    fn passing_moves_priority_to_the_next_player_in_turn_order() {
        let mut game = MockGame::builder().build();
        let p1 = game.p1;
        let p2 = game.p2;

        game.pass_priority();

        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(priority.player, p2);
        assert_eq!(priority.consecutive_passes, 1);
        assert_ne!(priority.player, p1);
    }

    #[test]
    fn a_four_player_game_passes_priority_in_a_ring() {
        let mut game = MockGame::builder().player_count(4).build();
        let expected = [game.p(2), game.p(3), game.p(4)];

        for next in expected {
            game.pass_priority();
            assert_eq!(game.app.world().resource::<turns::Priority>().player, next);
        }
    }

    #[test]
    fn the_caster_keeps_priority_after_casting_a_spell() {
        let mut game = MockGame::builder()
            .hand(Seat::P2, [lightning_bolt()])
            .mana_pool(Seat::P2, mana![R])
            .build();
        let p2 = game.p2;
        let bolt = game.find(p2, HAND, "Lightning Bolt").unwrap();

        game.pass_priority();
        game.act(p2, actions::Action::Cast(bolt));

        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(priority.player, p2);
        assert_eq!(priority.consecutive_passes, 0);
    }

    #[test]
    fn a_player_without_priority_cannot_pass() {
        let mut game = MockGame::builder().build();
        let p2 = game.p2;

        game.act(p2, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p2 })
        );
        assert_eq!(
            game.app.world().resource::<turns::Priority>().player,
            game.p1
        );
    }
}
