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

    #[must_use]
    pub fn legal<R: rules::traits::legality_rule::LegalityRule>(&self, context: &R::Ctx) -> bool {}

    #[must_use]
    pub fn performs(&self, entity: Entity, step: rules::StepId) -> bool {}

    #[must_use]
    pub fn demanded<R: rules::traits::legality_rule::LegalityRule>(&self, entity: Entity) -> bool {}
}

#[atomic_delegates]
impl Rules<'_, '_> {
    pub(crate) fn mods(&self, entity: Entity) -> Vec<effects::Change> {}
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::TAP_ATTACKERS;
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
                assert!(answers.performs(*bears, TAP_ATTACKERS));
                assert!(!answers.performs(*sovereign, TAP_ATTACKERS));
            })
            .unwrap();
        Ok(())
    }
}
