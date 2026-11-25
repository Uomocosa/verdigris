use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Query, Res, ResMut, not, on_message};

use crate::GameMut;
use crate::STACK;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::stack;
use crate::turns;
use crate::zones;

/// 514.3a At this point, the game checks to see if any state-based actions would be performed
/// and/or any triggered abilities are waiting to be put onto the stack (including those that
/// trigger “at the beginning of the next cleanup step”). If so, those state-based actions are
/// performed, then those triggered abilities are put on the stack, then the active player gets
/// priority. Players may cast spells and activate abilities. Once the stack is empty and all
/// players pass in succession, another cleanup step begins.
pub fn r514_3a_cleanup_reentry() -> ScheduleConfigs<ScheduleSystem> {
    (
        check_for_a_priority_window.run_if(not(turns::Step::Cleanup.begins())),
        repeat_cleanup_step.run_if(on_message::<turns::AllPlayersPassed>),
    )
        .chain()
}

fn check_for_a_priority_window(
    mut game: GameMut,
    reentry: ResMut<turns::CleanupReentry>,
    triggers: Res<stack::TriggerQueue>,
    performed: Res<StateBasedActionsPerformed>,
    pending: Res<actions::PendingChoice>,
    zones: zones::ZoneLookup,
    stack_contents: Query<&zones::ZoneContents>,
) {
    let reentry = reentry.into_inner();
    let triggers = triggers.into_inner();
    let performed = performed.into_inner();
    let pending = pending.into_inner();
    if game.step() != turns::Step::Cleanup {
        return;
    }
    if reentry.is_priority_window || reentry.should_repeat {
        return;
    }
    if pending.is_discard() {
        return;
    }
    let stack_is_empty = zones
        .find(STACK, None)
        .and_then(|stack| stack_contents.get(stack).ok())
        .is_none_or(|contents| contents.is_empty());
    if **performed || !triggers.is_empty() || !stack_is_empty {
        reentry.is_priority_window = true;
        game.give_priority_to_active_player();
    }
}

fn repeat_cleanup_step(reentry: ResMut<turns::CleanupReentry>, mut game: GameMut) {
    let reentry = reentry.into_inner();
    if !reentry.is_priority_window {
        return;
    }
    game.give_priority_to_active_player();
    reentry.should_repeat = true;
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::CLEANUP;
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_cleanup_trigger_gives_priority_and_starts_another_cleanup_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([WAYLAY]).mana_pool(mana![2, W]), P2]);
        let [waylay] = game.player(p1).cards_in(HAND).exactly::<1>(WAYLAY)?;
        game.play([p1.cast(waylay).pass_priority(), p2.pass_priority()])?;
        let [knight_1, knight_2, knight_3] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<3>(KNIGHT_TOKEN)?;

        game.play([p1.pass_to(CLEANUP), p2.pass_to(CLEANUP)])?;
        assert_eq!(game.turn().step, turns::Step::Cleanup);
        assert_eq!(game.priority().holder, Some(p1.id));

        game.play([p1.order_triggers([knight_1, knight_2, knight_3])])?;
        assert_eq!(game.trigger_sources().len(), 3);
        for _ in 0..3 {
            game.play([p1.pass_priority(), p2.pass_priority()])?;
        }
        assert_eq!(game.trigger_sources().len(), 0);

        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.turn().step, turns::Step::Cleanup);
        Ok(())
    }
}
