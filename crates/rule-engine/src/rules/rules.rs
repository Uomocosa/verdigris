use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::resource::IsResource;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, EntityRef, Query, Without};

use crate::cards;
use crate::effects;
use crate::rules;

#[derive(SystemParam, Clone, Copy)]
pub struct Rules<'w, 's> {
    pub definitions: Query<'w, 's, &'static cards::CardDefinition>,
    pub effects: Query<'w, 's, &'static effects::ContinuousEffect>,
    pub entities: Query<'w, 's, EntityRef<'static>, Without<IsResource>>,
    pub registry: Query<'w, 's, &'static rules::PredicateRegistry>,
}

#[atomic_delegates]
impl Rules<'_, '_> {
    pub fn ask<Q: rules::traits::rule_question::RuleQuestion>(&self, entity: Entity) -> Q::Answer {}

    #[must_use]
    pub fn characteristics(&self, entity: Entity) -> cards::Characteristics {}

    #[must_use]
    pub fn eval(
        &self,
        predicate: &rules::Predicate,
        subject: Entity,
        other: Option<Entity>,
    ) -> bool {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([FELIDAR_SOVEREIGN]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [sovereign] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.ask::<rules::TapsWhenAttacking>(*bears));
                assert!(!answers.ask::<rules::TapsWhenAttacking>(*sovereign));
            })
            .unwrap();
        Ok(())
    }
}
