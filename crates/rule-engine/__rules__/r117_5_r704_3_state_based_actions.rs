use bevy::prelude::World;

use crate::MAX_STATE_BASED_PASSES;
use crate::StateBasedActions;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::rules;
use crate::stack;

/// 117.5. Each time a player would get priority, the game first performs all applicable
/// state-based actions as a single event (see rule 704, “State-Based Actions”), then repeats this process until no state-based actions are performed. Then triggered abilities are put on the stack (see rule 603, “Handling Triggered Abilities”). These steps repeat in order until no further state-based actions are performed and no abilities trigger. Then the player who would have received priority does so.
///
/// 704.3. Whenever a player would get priority (see rule 117, “Timing and Priority”), the game checks for any of the listed conditions for state-based actions, then performs all applicable state-based actions simultaneously as a single event. If any state-based actions are performed as a result of a check, the check is repeated; otherwise all triggered abilities that are waiting to be put on the stack are put on the stack, then the check is repeated. Once no more state-based actions have been performed as the result of a check and no triggered abilities are waiting to be put on the stack, the appropriate player gets priority. This process also occurs during the cleanup step (see rule 514), except that if no state-based actions are performed as the result of the step’s first check and no triggered abilities are waiting to be put on the stack, then no player gets priority and the step ends.
pub fn r117_5_r704_3_state_based_actions(world: &mut World) {
    for _ in 0..MAX_STATE_BASED_PASSES {
        **world.resource_mut::<StateBasedActionsPerformed>() = false;
        world.run_schedule(StateBasedActions);
        if !**world.resource::<StateBasedActionsPerformed>() {
            if world.resource::<stack::TriggerQueue>().is_empty() {
                return;
            }
            if matches!(
                (*world.resource::<actions::PendingChoice>()).as_ref(),
                Some(actions::Choice::OrderTriggers(_))
            ) {
                return;
            }
            rules::r603_3_triggers_on_stack::r603_3_triggers_on_stack(world);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::players;
    use crate::testing::*;

    #[test]
    fn player_at_zero_life_loses_and_opponent_wins() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2.life(3)]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.app.world().get::<players::Lost>(*p2).is_some());
        assert!(game.app.world().get::<players::Won>(*p1).is_some());
        Ok(())
    }
}
