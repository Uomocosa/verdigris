#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Question {
    TapsWhenAttacking,
    IgnoresSummoningSickness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combine {
    LatestTimestamp,
}
