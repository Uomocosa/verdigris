use bevy::prelude::Component;

use crate::cards;
use crate::rules;
use crate::turns;
use crate::zones;

#[must_use]
pub fn has<C: Component>() -> rules::Condition {
    rules::Condition::Has(rules::ComponentRef::of::<C>())
}

#[must_use]
pub const fn card_type(card: cards::CardType) -> rules::Condition {
    rules::Condition::CardType(card)
}

#[must_use]
pub const fn at_least(characteristic: rules::Characteristic, threshold: i32) -> rules::Condition {
    rules::Condition::Characteristic(characteristic, rules::Comparison::AtLeast, threshold)
}

#[must_use]
pub const fn at_most(characteristic: rules::Characteristic, threshold: i32) -> rules::Condition {
    rules::Condition::Characteristic(characteristic, rules::Comparison::AtMost, threshold)
}

#[must_use]
pub const fn equal_to(characteristic: rules::Characteristic, threshold: i32) -> rules::Condition {
    rules::Condition::Characteristic(characteristic, rules::Comparison::EqualTo, threshold)
}

#[must_use]
pub fn points_to<C: rules::traits::entity_component::EntityComponent>(
    role: impl Into<rules::ArgRef>,
) -> rules::Condition {
    rules::Condition::PointsTo(rules::ComponentRef::relation::<C>(), role.into())
}

#[must_use]
pub fn is(role: impl Into<rules::ArgRef>) -> rules::Condition {
    rules::Condition::Is(role.into())
}

#[must_use]
pub fn in_zone(zone: zones::Zone, role: impl Into<rules::ArgRef>) -> rules::Condition {
    rules::Condition::InZone(zone, role.into())
}

#[must_use]
pub const fn step_is(step: turns::Step) -> rules::Condition {
    rules::Condition::StepIs(step)
}

#[must_use]
pub fn main_phase() -> rules::Condition {
    rules::Condition::Any(vec![
        rules::Condition::StepIs(turns::Step::PrecombatMain),
        rules::Condition::StepIs(turns::Step::PostcombatMain),
    ])
}

#[must_use]
pub const fn stack_empty() -> rules::Condition {
    rules::Condition::StackEmpty
}

#[must_use]
pub fn active_player(role: impl Into<rules::ArgRef>) -> rules::Condition {
    rules::Condition::ActivePlayer(role.into())
}

#[must_use]
pub fn has_priority(role: impl Into<rules::ArgRef>) -> rules::Condition {
    rules::Condition::HasPriority(role.into())
}

#[must_use]
pub const fn count(id: rules::ArgId) -> rules::Count {
    rules::Count(id)
}
