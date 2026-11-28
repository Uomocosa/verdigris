use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, ResMut, With};

use crate::Attempt;
use crate::MINUS_ONE_MINUS_ONE;
use crate::PLUS_ONE_PLUS_ONE;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::cards;
use crate::counters;
use crate::events;
use crate::rules;

const CAUSE: rules::EventId = rules::EventId("704.5q/counter-cancellation");

/// 704.5q If a permanent has both a +1/+1 counter and a -1/-1 counter on it, N +1/+1 and N -1/-1
/// counters are removed from it, where N is the smaller of the number of +1/+1 and -1/-1 counters
/// on it.
///
/// # See also
///
/// 122.3. If a permanent has both a +1/+1 counter and a -1/-1 counter on it, N +1/+1 and N -1/-1
/// counters are removed from it as a state-based action, where N is the smaller of the number of
/// +1/+1 and -1/-1 counters on it. See rule 704.
pub fn r704_5q_counter_cancellation() -> ScheduleConfigs<ScheduleSystem> {
    (
        announce_counter_cancellation,
        Attempt::<events::RemoveCounters>::resolved_by(remove_counters),
    )
        .chain()
}

fn announce_counter_cancellation(
    mut removals: MessageWriter<Attempt<events::RemoveCounters>>,
    mut performed: ResMut<StateBasedActionsPerformed>,
    permanents: Query<(Entity, &counters::Counters), With<cards::OnBattlefield>>,
) {
    let removals_in_permanent_order: Vec<events::RemoveCounters> = permanents
        .iter()
        .flat_map(|(recipient, held)| {
            let amount = held.get(PLUS_ONE_PLUS_ONE).min(held.get(MINUS_ONE_MINUS_ONE));
            [PLUS_ONE_PLUS_ONE, MINUS_ONE_MINUS_ONE].into_iter().map(move |kind| {
                events::RemoveCounters {
                    recipient,
                    kind,
                    amount,
                    cause: CAUSE,
                }
            })
        })
        .filter(|removal| removal.amount > 0)
        .collect();
    if !removals_in_permanent_order.is_empty() {
        **performed = true;
    }
    removals.write_batch(removals_in_permanent_order.into_iter().map(Attempt));
}

fn remove_counters(
    mut removals: MessageReader<Resolve<events::RemoveCounters>>,
    mut counted: Query<&mut counters::Counters>,
) {
    for removal in removals.read() {
        if let Ok(mut held) = counted.get_mut(removal.recipient) {
            held.remove(removal.kind, removal.amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::PLUS_ONE_PLUS_ONE;
    use crate::counters;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn unequal_counter_counts_leave_the_remainder() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([PLAGUE_STINGER]),
            P2.battlefield([GIANT_SPIDER, FOREST, FOREST]).hand([BATTLEGROWTH, BATTLEGROWTH]),
        ]);
        let [stinger] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(PLAGUE_STINGER)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        let [forest_1, forest_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<2>(FOREST)?;
        let [growth_1, growth_2] = game.player(p2).cards_in(HAND).exactly::<2>(BATTLEGROWTH)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([stinger]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(spider, stinger)])])?;
        game.play([
            p2.activate_ability(forest_1.ability()).cast(growth_1).targeting(spider),
            p2.activate_ability(forest_2.ability())
                .cast(growth_2)
                .targeting(spider)
                .pass_priority(),
            p1.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        let mut expected = counters::Counters::default();
        expected.add(PLUS_ONE_PLUS_ONE, 1);
        assert_eq!(game.get::<counters::Counters>(spider), Some(expected));
        Ok(())
    }
}
