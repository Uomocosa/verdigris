use bevy::prelude::{Entity, MessageReader, Query, Res, ResMut, With};

use crate::abilities;
use crate::cards;
use crate::players;
use crate::stack;
use crate::turns;

/// 603.4. A triggered ability may read “When/Whenever/At [trigger event], if [condition],
/// [effect].” When the trigger event occurs, the ability checks whether the stated condition is
/// true. The ability triggers only if it is; otherwise it does nothing. If the ability triggers,
/// it checks the stated condition again as it resolves. If the condition isn’t true at that time,
/// the ability is removed from the stack and does nothing. Note that this mirrors the check for
/// legal targets. This rule is referred to as the “intervening ‘if’ clause” rule. (The word “if”
/// has only its normal English meaning anywhere else in the text of a card; this rule only
/// applies to an “if” that immediately follows a trigger condition.)
///
/// # See also
///
/// Example: Felidar Sovereign reads, “At the beginning of your upkeep, if you have 40 or more
/// life, you win the game.” Its controller’s life total is checked as that player’s upkeep
/// begins. If that player has 39 or less life, the ability doesn’t trigger at all. If that
/// player has 40 or more life, the ability triggers and goes on the stack. As the ability
/// resolves, that player’s life total is checked again. If that player has 39 or less life at
/// this time, the ability is removed from the stack and has no effect. If that player has 40 or
/// more life at this time, the ability resolves and that player wins the game.
pub fn r603_4_intervening_if(
    mut steps: MessageReader<turns::StepStarted>,
    mut queue: ResMut<stack::TriggerQueue>,
    sources: Query<(Entity, &cards::Owner, &cards::TriggeredAtStep), With<cards::OnBattlefield>>,
    lives: Query<(Entity, &players::Life)>,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for step in steps.read() {
        for (source, owner, at_step) in sources.iter() {
            let controller = **owner;
            let triggered = match at_step.event {
                abilities::TriggerEvent::BeginningOfStep(expected, who) => {
                    **step == expected && {
                        let players: Vec<Entity> = lives.iter().map(|(player, _)| player).collect();
                        who.resolve(controller, &players).contains(&*turn.active)
                    }
                }
                abilities::TriggerEvent::BeginningOfNextCleanup => {
                    **step == turns::Step::Cleanup && turn.active == players::PlayerId(controller)
                }
                abilities::TriggerEvent::Enters(_)
                | abilities::TriggerEvent::Dies
                | abilities::TriggerEvent::Attacks
                | abilities::TriggerEvent::Blocks
                | abilities::TriggerEvent::BecomesBlocked
                | abilities::TriggerEvent::DealtCombatDamageToPlayer => false,
            };
            if !triggered {
                continue;
            }
            let ability = **at_step;
            if let Some(condition) = ability.intervening_if
                && !condition.holds(controller, &lives)
            {
                continue;
            }
            queue.push(stack::PendingTrigger {
                source,
                controller,
                ability,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn felidar_sovereign_wins_at_forty_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::Untap)
            .players([P1.battlefield([FELIDAR_SOVEREIGN]).life(40), P2]);
        let [sovereign] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(sovereign));
        game.play([p1.pass_priority(), p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.has_won(p1));
        Ok(())
    }

    #[test]
    fn felidar_sovereign_does_not_trigger_below_forty_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::Untap)
            .players([P1.battlefield([FELIDAR_SOVEREIGN]).life(39), P2]);
        let [sovereign] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        game.play([p1.pass_priority(), p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(sovereign));
        assert!(!game.has_won(p1));
        assert_eq!(game.trigger_sources().len(), 0);
        Ok(())
    }
}
