use crate::turns;

#[must_use]
pub const fn beginning_of(step: turns::Step) -> turns::BeginningOf {
    turns::BeginningOf(step)
}
