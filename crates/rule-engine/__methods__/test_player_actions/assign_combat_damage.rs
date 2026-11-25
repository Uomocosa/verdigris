use crate::actions;
use crate::testing;

pub fn assign_combat_damage<const N: usize>(
    player_actions: testing::TestPlayerActions,
    assignments: [(testing::Targets, u32); N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::AssignCombatDamage(
        assignments
            .into_iter()
            .flat_map(|(target, amount)| {
                target.into_entities().into_iter().map(move |entity| actions::DamageAssignment {
                    target: entity,
                    amount,
                })
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::assign_combat_damage;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();

        assert_eq!(
            assign_combat_damage(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    caster,
                    testing::PlayerSeat(1)
                )),
                [
                    (testing::Targets::from(first), 1),
                    (testing::Targets::from(second), 1),
                ]
            )
            .steps,
            vec![testing::PlayStep::Act(
                caster,
                actions::Action::AssignCombatDamage(vec![
                    actions::DamageAssignment {
                        target: first,
                        amount: 1,
                    },
                    actions::DamageAssignment {
                        target: second,
                        amount: 1,
                    },
                ])
            )]
        );
    }
}
