use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::resource::IsResource;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, EntityRef, Query, Without};

use crate::cards;
use crate::effects;
use crate::keywords;
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
    pub fn eval<E: rules::traits::event_args::EventArgs>(
        &self,
        condition: &rules::Condition,
        event: &E,
        viewpoint: Option<bevy::prelude::Entity>,
    ) -> bool {
    }

    #[must_use]
    pub fn legality<E: rules::traits::event::Event>(&self, event: &E) -> rules::Legality {}

    #[must_use]
    pub fn performs<E: rules::traits::event::Event>(&self, event: &E) -> bool {}

    #[must_use]
    pub fn replacement<E: rules::traits::event::Event>(&self, event: &E) -> Option<rules::Then> {}

    #[must_use]
    pub fn replaced<E: rules::traits::event::Event>(
        &self,
        result: rules::ResultId,
        event: &E,
    ) -> Option<rules::Then> {
    }

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

    #[must_use]
    pub fn resolve<E: rules::traits::event_args::EventArgs>(
        &self,
        reference: rules::ArgRef,
        event: &E,
    ) -> Option<rules::ArgValue> {
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
    pub(crate) fn holder_changes<E: rules::traits::event::Event>(
        &self,
        event: &E,
        target: rules::Target,
    ) -> Vec<rules::Change> {
    }

    pub(crate) fn eval_has(
        &self,
        component: &rules::ComponentRef,
        viewpoint: Option<Entity>,
    ) -> bool {
    }

    pub(crate) fn eval_has_keyword(
        &self,
        id: keywords::KeywordId,
        viewpoint: Option<Entity>,
    ) -> bool {
    }

    pub(crate) fn eval_points_to<E: rules::traits::event_args::EventArgs>(
        &self,
        component: &rules::ComponentRef,
        reference: rules::ArgRef,
        event: &E,
        viewpoint: Option<Entity>,
    ) -> bool {
    }

    pub(crate) fn eval_card_type(&self, card: cards::CardType, viewpoint: Option<Entity>) -> bool {}

    pub(crate) fn eval_characteristic(
        &self,
        characteristic: rules::Characteristic,
        comparison: rules::Comparison,
        threshold: i32,
        viewpoint: Option<Entity>,
    ) -> bool {
    }

    pub(crate) fn eval_in_zone<E: rules::traits::event_args::EventArgs>(
        &self,
        zone: zones::Zone,
        reference: rules::ArgRef,
        event: &E,
        viewpoint: Option<Entity>,
    ) -> bool {
    }

    pub(crate) fn mods(&self, entity: Entity) -> Vec<effects::Modification> {}

    pub(crate) fn remaining_toughness(&self, subject: Entity) -> u32 {}

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
    use crate::LIBRARY;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]).library([GRIZZLY_BEARS]),
            P2.battlefield([FELIDAR_SOVEREIGN]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [sovereign] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        let [library_bears] = game.player(p1).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let damage = events::DealDamage {
            source: *bears,
            recipient: *p2.id,
            amount: 3,
        };
        let library_damage = events::DealDamage {
            source: *library_bears,
            recipient: *p2.id,
            amount: 3,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.performs(&events::TapAttacker { attacker: *bears }));
                assert!(!answers.performs(&events::TapAttacker {
                    attacker: *sovereign,
                }));
                assert_eq!(
                    answers.resolve(events::DealDamage::SOURCE.controller(), &damage),
                    Some(rules::ArgValue::Player(p1.id))
                );
                assert_eq!(
                    answers.resolve(events::DealDamage::RECIPIENT.controller(), &damage),
                    Some(rules::ArgValue::Player(p2.id))
                );
                assert_eq!(
                    answers.resolve(rules::ArgRef::Arg(events::DealDamage::AMOUNT), &damage),
                    Some(rules::ArgValue::Amount(3))
                );
                assert_eq!(
                    answers.resolve(events::DealDamage::SOURCE.controller(), &library_damage),
                    Some(rules::ArgValue::Player(p1.id))
                );
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
