use bevy::prelude::{Commands, Entity, MessageReader, Query, Res, With};

use crate::cards;
use crate::turns;

/// 302.6. A creature’s activated ability with the tap symbol or the untap symbol in its activation cost can’t be activated unless the creature has been under its controller’s control continuously since their most recent turn began. A creature can’t attack unless it has been under its controller’s control continuously since their most recent turn began. This rule is informally called the “summoning sickness” rule.
pub fn r302_6_summoning_sickness(
    mut step_started: MessageReader<turns::StepStarted>,
    mut commands: Commands,
    turn: Res<turns::Turn>,
    sick: Query<(Entity, &cards::Owner), With<cards::SummoningSick>>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::Untap {
            continue;
        }
        for (entity, owner) in sick.iter() {
            if **owner == turn.active {
                commands.entity(entity).remove::<cards::SummoningSick>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn summoning_sickness_lifts_at_the_controllers_next_untap() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([WALKING_CORPSE]).mana_pool(mana![1, B]), P2]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE)?;
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(corpse));
        assert!(game.has::<cards::SummoningSick>(corpse));

        game.play([p1.end_turn(), p2.end_turn()])?;

        assert_eq!(game.turn().number, 2);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(corpse));
        assert!(game.has::<cards::SummoningSick>(corpse));

        game.play([p2.end_turn(), p1.end_turn()])?;
        game.play([p1.pass_to(turns::Step::Upkeep)])?;

        assert_eq!(game.turn().number, 3);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(corpse));
        assert!(!game.has::<cards::SummoningSick>(corpse));
        Ok(())
    }
}
