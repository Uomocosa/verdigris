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
    pub state: Query<'w, 's, &'static turns::TurnState>,
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
        condition: &rules::Condition,
        subject: Entity,
        other: Option<Entity>,
        player: Option<Entity>,
    ) -> bool {
    }

    #[must_use]
    pub fn legality<E: rules::traits::event::Event>(&self, event: &E) -> rules::Legality {}

    #[must_use]
    pub fn performs<E: rules::traits::event::Event>(&self, event: &E) -> bool {}

    #[must_use]
    pub fn replacement<E: rules::traits::event::Event>(&self, event: &E) -> Option<rules::Then> {}

    #[must_use]
    pub fn hooks<E: rules::traits::event::Event>(&self, event: &E) -> Vec<rules::Then> {}

    #[must_use]
    pub fn changes(&self, entity: Entity, target: rules::Target) -> Vec<rules::Change> {}

    #[must_use]
    pub fn value<V: rules::traits::value_rule::ValueRule>(&self, entity: Entity) -> rules::Limit {}

    #[must_use]
    pub fn defines<E: rules::traits::event::Event>(
        &self,
        definition: &rules::Definition,
        event: &E,
    ) -> bool {
    }
}

impl Rules<'_, '_> {
    #[must_use]
    pub fn legal<E: rules::traits::event::Event>(&self, event: &E) -> bool {
        let outcome = self.legality::<E>(event);
        outcome == rules::Legality::Legal
    }
}

#[rustfmt::skip]
impl Rules<'_, '_> {
    #[must_use]
    pub fn required<E: rules::traits::event::Event>(&self, entity: Entity) -> bool {
        self.changes(entity, rules::Target::event::<E>()).contains(&rules::Change::Must)
    }
}

#[atomic_delegates]
impl Rules<'_, '_> {
    pub(crate) fn mods(&self, entity: Entity) -> Vec<effects::Modification> {}

    pub(crate) fn waived<E: rules::traits::event::Event>(
        &self,
        event: &E,
        target: rules::Target,
        unwaivable: bool,
    ) -> bool {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
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
                assert!(answers.performs(&events::TapAttacker {
                    attacker: *bears,
                    player: p1.id,
                }));
                assert!(!answers.performs(&events::TapAttacker {
                    attacker: *sovereign,
                    player: p2.id,
                }));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn required_holds_for_must_and_fails_otherwise() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.battlefield([TEST_RULE_WAIVER, GRIZZLY_BEARS]), P2]);
        let [waiver] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RULE_WAIVER)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.pass_priority();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.required::<TestRule>(*waiver));
                assert!(!answers.required::<TestRule>(*bears));
            })
            .unwrap();
        Ok(())
    }
}
