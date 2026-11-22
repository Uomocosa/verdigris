#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Question {
    TapsWhenAttacking,
    IgnoresSummoningSickness,
    MustAttack,
    MustBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combine {
    LatestTimestamp,
}
