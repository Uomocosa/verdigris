use crate::cards;
use crate::rules;
use crate::turns;
use crate::zones;

#[must_use]
pub const fn has<C: rules::traits::keyed_component::KeyedComponent>() -> rules::Predicate {
    rules::Predicate::Has(C::KEY)
}

#[must_use]
pub const fn card_type(card: cards::CardType) -> rules::Predicate {
    rules::Predicate::CardType(card)
}

#[must_use]
pub const fn at_least(characteristic: rules::Characteristic, threshold: i32) -> rules::Predicate {
    rules::Predicate::Characteristic(characteristic, rules::Comparison::AtLeast, threshold)
}

#[must_use]
pub const fn at_most(characteristic: rules::Characteristic, threshold: i32) -> rules::Predicate {
    rules::Predicate::Characteristic(characteristic, rules::Comparison::AtMost, threshold)
}

#[must_use]
pub const fn equal_to(characteristic: rules::Characteristic, threshold: i32) -> rules::Predicate {
    rules::Predicate::Characteristic(characteristic, rules::Comparison::EqualTo, threshold)
}

#[must_use]
pub fn other(predicate: rules::Predicate) -> rules::Predicate {
    rules::Predicate::Of(rules::Role::Other, Box::new(predicate))
}

#[must_use]
pub const fn points_to<C: rules::traits::entity_component::EntityComponent>(
    role: rules::Role,
) -> rules::Predicate {
    rules::Predicate::PointsTo(C::KEY, role)
}

#[must_use]
pub const fn in_zone(zone: zones::Zone, role: rules::Role) -> rules::Predicate {
    rules::Predicate::InZone(zone, role)
}

#[must_use]
pub const fn step_is(step: turns::Step) -> rules::Predicate {
    rules::Predicate::StepIs(step)
}

#[must_use]
pub fn main_phase() -> rules::Predicate {
    rules::Predicate::Any(vec![
        rules::Predicate::StepIs(turns::Step::PrecombatMain),
        rules::Predicate::StepIs(turns::Step::PostcombatMain),
    ])
}

#[must_use]
pub const fn stack_empty() -> rules::Predicate {
    rules::Predicate::StackEmpty
}

#[must_use]
pub const fn active_player(role: rules::Role) -> rules::Predicate {
    rules::Predicate::ActivePlayer(role)
}

#[must_use]
pub const fn has_priority(role: rules::Role) -> rules::Predicate {
    rules::Predicate::HasPriority(role)
}

#[must_use]
pub const fn event_field(
    field: rules::Field,
    comparison: rules::Comparison,
    threshold: u32,
) -> rules::Predicate {
    rules::Predicate::EventField(field, comparison, threshold)
}
