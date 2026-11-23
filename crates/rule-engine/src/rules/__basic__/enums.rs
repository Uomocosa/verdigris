#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Question {
    TapsWhenAttacking,
    IgnoresSummoningSickness,
    MustAttack,
    MustBlock,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combine {
    LatestTimestamp,
}
