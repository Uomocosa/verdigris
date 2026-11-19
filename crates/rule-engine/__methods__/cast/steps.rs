use crate::actions;
use crate::testing;

pub fn steps(cast: &testing::Cast) -> Vec<testing::Step> {
    let mut steps = vec![testing::Step::Act(
        cast.caster,
        actions::Action::Cast(cast.spell),
    )];
    if !cast.targets.is_empty() {
        steps.push(testing::Step::Act(
            cast.caster,
            actions::Action::ChooseTargets(cast.targets.clone()),
        ));
    }
    if cast.should_resolve {
        steps.push(testing::Step::PassPriorityAll);
    }
    steps
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::steps;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let cast = testing::Cast::from_caster_and_spell(caster, spell);

        assert_eq!(
            steps(&cast),
            vec![testing::Step::Act(caster, actions::Action::Cast(spell))]
        );
    }
}
