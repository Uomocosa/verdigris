#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Characteristic {
    Power,
    Toughness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    AtLeast,
    AtMost,
    EqualTo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Subject,
    Other,
}
