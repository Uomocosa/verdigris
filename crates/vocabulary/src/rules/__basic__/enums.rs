#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgKind {
    Entity,
    Entities,
    Player,
    Amount,
    Event,
}
