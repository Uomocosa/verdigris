use crate::cards;
use crate::rules;
use crate::turns;
use crate::zones;

#[must_use]
pub const fn has<C: rules::traits::keyed_component::KeyedComponent>() -> rules::Condition {
    rules::Condition::Has(C::KEY)
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
pub fn other(condition: rules::Condition) -> rules::Condition {
    rules::Condition::Of(rules::Role::Other, Box::new(condition))
}

#[must_use]
pub const fn points_to<C: rules::traits::entity_component::EntityComponent>(
    role: rules::Role,
) -> rules::Condition {
    rules::Condition::PointsTo(C::KEY, role)
}

#[must_use]
pub const fn is(role: rules::Role) -> rules::Condition {
    rules::Condition::Is(role)
}

#[must_use]
pub const fn in_zone(zone: zones::Zone, role: rules::Role) -> rules::Condition {
    rules::Condition::InZone(zone, role)
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
pub const fn active_player(role: rules::Role) -> rules::Condition {
    rules::Condition::ActivePlayer(role)
}

#[must_use]
pub const fn has_priority(role: rules::Role) -> rules::Condition {
    rules::Condition::HasPriority(role)
}

#[must_use]
pub const fn event_field(
    field: rules::Field,
    comparison: rules::Comparison,
    threshold: u32,
) -> rules::Condition {
    rules::Condition::EventField(field, comparison, threshold)
}
