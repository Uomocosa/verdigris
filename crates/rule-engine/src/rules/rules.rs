use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::resource::IsResource;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, EntityRef, Query, Without};

use crate::cards;
use crate::effects;
use crate::rules;
use crate::turns;
use crate::zones;

#[derive(SystemParam, Clone, Copy)]
pub struct Rules<'w, 's> {
    pub definitions: Query<'w, 's, &'static cards::CardDefinition>,
    pub effects: Query<'w, 's, &'static effects::ContinuousEffect>,
    pub entities: Query<'w, 's, EntityRef<'static>, Without<IsResource>>,
    pub registry: Query<'w, 's, &'static rules::PredicateRegistry>,
    pub turn: Query<'w, 's, &'static turns::Turn>,
    pub priority: Query<'w, 's, &'static turns::Priority>,
    pub zones: zones::ZoneLookup<'w, 's>,
    pub in_zone: Query<'w, 's, &'static zones::InZone>,
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
        player: Option<Entity>,
    ) -> bool {
    }

    #[must_use]
    pub fn legality<R: rules::traits::legality_rule::LegalityRule>(
        &self,
        context: &R::Ctx,
    ) -> rules::Legality {
    }

    #[must_use]
    pub fn performs(&self, entity: Entity, step: rules::ActionId) -> bool {}

    #[must_use]
    pub fn required<R: rules::traits::legality_rule::LegalityRule>(&self, entity: Entity) -> bool {}
}

impl Rules<'_, '_> {
    #[must_use]
    pub fn legal<R: rules::traits::legality_rule::LegalityRule>(&self, context: &R::Ctx) -> bool {
        let outcome = self.legality::<R>(context);
        outcome == rules::Legality::Legal
    }
}

#[atomic_delegates]
impl Rules<'_, '_> {
    pub(crate) fn mods(&self, entity: Entity) -> Vec<effects::Modification> {}
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
