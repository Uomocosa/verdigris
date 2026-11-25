use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::on_message;

use crate::GameMut;
use crate::HooksDone;
use crate::events;

/// 103.8. The starting player takes their first turn.
pub fn r103_8_first_turn() -> ScheduleConfigs<ScheduleSystem> {
    first_turn.run_if(on_message::<HooksDone<events::OpeningHandAction>>)
}

fn first_turn(mut game: GameMut) {
    game.give_priority_to_active_player();
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn passing_priority_while_an_opening_hand_action_is_waiting_is_rejected() {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION]), P2]);

        game.act(p1, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p1.id })
        );
        assert_eq!(game.priority().holder, None);
    }
}
