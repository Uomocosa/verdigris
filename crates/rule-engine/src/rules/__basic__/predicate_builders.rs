use crate::rules;

#[must_use]
pub const fn has<C: rules::traits::keyed_component::KeyedComponent>() -> rules::Predicate {
    rules::Predicate::Has(C::KEY)
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
