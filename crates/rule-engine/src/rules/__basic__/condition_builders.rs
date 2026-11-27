use bevy::prelude::Component;

use crate::cards;
use crate::keywords;
use crate::rules;
use crate::turns;
use crate::zones;

#[must_use]
pub fn has<C: Component>() -> rules::Condition {
    rules::Condition::Has(rules::ComponentRef::of::<C>())
}

#[must_use]
pub const fn has_keyword(id: keywords::KeywordId) -> rules::Condition {
    rules::Condition::HasKeyword(id)
}

#[must_use]
pub const fn card_type(card: cards::CardType) -> rules::Condition {
    rules::Condition::CardType(card)
}

#[must_use]
pub const fn power() -> rules::Characteristic {
    rules::Characteristic::Power
}

#[must_use]
pub const fn toughness() -> rules::Characteristic {
    rules::Characteristic::Toughness
}

#[must_use]
pub const fn exactly(threshold: i32) -> rules::Comparison {
    rules::Comparison::Exactly(threshold)
}

#[must_use]
pub const fn greater_than(threshold: i32) -> rules::Comparison {
    rules::Comparison::GreaterThan(threshold)
}

#[must_use]
pub const fn less_than(threshold: i32) -> rules::Comparison {
    rules::Comparison::LessThan(threshold)
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
pub const fn life() -> rules::PlayerQuantity {
    rules::PlayerQuantity::Life
}

#[must_use]
pub const fn poison_counters() -> rules::PlayerQuantity {
    rules::PlayerQuantity::PoisonCounters
}

#[must_use]
pub const fn cards_in_hand() -> rules::PlayerQuantity {
    rules::PlayerQuantity::CardsInHand
}

#[must_use]
pub const fn cards_in_library() -> rules::PlayerQuantity {
    rules::PlayerQuantity::CardsInLibrary
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

#[must_use]
pub const fn each(players: rules::Players) -> rules::Each {
    rules::Each(players)
}

#[must_use]
pub const fn an(players: rules::Players) -> rules::An {
    rules::An(players)
}
